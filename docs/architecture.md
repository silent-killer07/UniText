# Architecture & Design

## The Three-Layer Model

UniText abstracts text into three distinct layers to solve Unicode's inherent ambiguity and complexity:

```text
┌─────────────────────────────────────────────────────────┐
│                   LAYER 3: VISUAL                        │
│    Emoji sequences, presentation forms, styling hints    │
│    (Separated from text — processed by renderers)        │
├─────────────────────────────────────────────────────────┤
│                   LAYER 2: MODIFIERS                     │
│    Diacritics, combining marks, script variants          │
│    (Pre-composed into canonical graphemes at encode)     │
├─────────────────────────────────────────────────────────┤
│                   LAYER 1: BASE TEXT                     │
│    Core characters from all scripts, digits, punctuation │
│    (Fixed-width grapheme slots with auxiliary index)     │
└─────────────────────────────────────────────────────────┘
```

### How each layer solves a specific Unicode problem:

| Layer | Unicode Problem It Solves |
|-------|--------------------------|
| **Layer 1 (Base Text)** | O(n) indexing → Fixed grapheme slots with index give O(1) access |
| **Layer 2 (Modifiers)** | Normalization ambiguity → All combining sequences pre-composed into canonical form at encode time |
| **Layer 3 (Visual)** | Emoji chaos → Emoji/ZWJ sequences live in a separate channel, don't corrupt text operations |

---

## The Grapheme Table (Core Data Structure)

Instead of storing text as a flat byte array (like UTF-8), UniText stores text as a **Grapheme Table** — an indexed array of grapheme entries.

```text
┌──────┬────────────────┬────────┬──────────┬───────────┐
│ Slot │ Canonical Form │ Script │ Category │ Visual ID │
├──────┼────────────────┼────────┼──────────┼───────────┤
│  0   │ U+0048         │ Latin  │ Letter   │ null      │
│  1   │ U+0065         │ Latin  │ Letter   │ null      │
│  2   │ U+006C         │ Latin  │ Letter   │ null      │
│  3   │ U+006C         │ Latin  │ Letter   │ null      │
│  4   │ U+006F         │ Latin  │ Letter   │ null      │
│  5   │ U+0020         │ Common │ Space    │ null      │
│  6   │ NFC(U+00E9)    │ Latin  │ Letter   │ null      │ ← "é" always stored as one unit
│  7   │ —              │ Emoji  │ Visual   │ ZWJ_0042  │ ← emoji stored as reference
└──────┴────────────────┴────────┴──────────┴───────────┘
```

**Key properties:**
- Each slot provides **O(1) random access** to human-perceived characters.
- The `Script` field enables **instant homograph detection** (mixed-script = suspicious).
- The `Canonical Form` is always NFC-normalized at write time → **no normalization ambiguity**.
- Emoji/ZWJ sequences are stored as **references to a Visual Table**, not inline → text operations skip them cleanly.

---

## The Normalizer (Encoding Pipeline)

When text enters the UniText system (from any source: UTF-8, UTF-16, user input, clipboard, etc.), it goes through a strict pipeline:

```text
Raw Input (any encoding)
    │
    ▼
┌──────────────────────┐
│ 1. DECODE            │  Convert from source encoding to Unicode code points
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ 2. NORMALIZE (NFC)   │  Collapse all combining sequences into precomposed form
│                      │  e + ́  → é  (always, no exceptions)
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ 3. SEGMENT           │  Run UAX #29 grapheme cluster segmentation
│                      │  Identify true "human characters"
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ 4. CLASSIFY          │  Tag each grapheme with Script + Category
│                      │  Detect mixed-script anomalies
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ 5. SEPARATE VISUAL   │  Extract emoji/ZWJ sequences to Visual Table
│                      │  Replace inline with Visual references
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ 6. INDEX             │  Build grapheme index for O(1) access
│                      │  Store in Grapheme Table
└──────────────────────┘
```

**This pipeline guarantees:**
- ✅ Every string that enters the system is in exactly one canonical form
- ✅ `"Café"` encoded two different ways in Unicode will be **identical** in UniText
- ✅ Emoji sequences are cleanly separated and won't break `length()`, `substring()`, `reverse()`
- ✅ Script information is always available for security checks
