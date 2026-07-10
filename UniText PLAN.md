# UniText: A Next-Generation Text Encoding Abstraction System

## The Problem

Every major encoding system in computing history has fundamental flaws:

| System | Fatal Flaw |
|--------|-----------|
| **ASCII** | English-only. 128 characters. Useless for 6+ billion non-English speakers. |
| **EBCDIC** | Non-contiguous alphabet. Dead relic of punch cards. |
| **ISO 8859-x** | Fragmented into 15+ incompatible regional "parts". Mojibake nightmare. |
| **UTF-8** | Variable-width (1-4 bytes). O(n) character indexing. Normalization ambiguity. |
| **UTF-16** | Variable-width (2 or 4 bytes). Worst of both worlds — neither compact nor fixed. |
| **UTF-32** | Fixed-width but 4 bytes per character. Wastes 75% memory for English/Latin text. |

Unicode itself (the character set, not the encoding) has deeper problems:
- **Normalization chaos**: `é` can be encoded two completely different ways that look identical
- **Homograph attacks**: Cyrillic `а` and Latin `a` are visually identical but different code points — enables phishing
- **Emoji scope creep**: 👨‍👩‍👧‍👦 is actually 7 code points glued together with invisible "Zero Width Joiners"
- **Han Unification**: Culturally distinct CJK characters forced into shared code points
- **Grapheme ≠ Code Point**: What humans see as "one character" can be 1 to 25+ code points

> [!IMPORTANT]
> **Reality Check**: Replacing Unicode globally is impossible — it's baked into every OS, database, programming language, and protocol on Earth. But what IS missing and absolutely buildable is a **smart abstraction layer** that sits on TOP of Unicode and fixes its problems for developers and users. Think of it like how TCP/IP sits on top of raw electrical signals — we don't replace the wires, we build intelligence on top.

---

## What We're Actually Building

**UniText** — An open-source library and toolkit that provides a "perfect text" abstraction layer over Unicode. It makes text behave the way humans expect, not the way bytes work.

### Core Principles
1. **Grapheme-First**: The atomic unit is what humans see as one character, not code points
2. **Canonical-Only**: Every character has exactly ONE internal representation — no ambiguity ever
3. **Security-Native**: Homograph/confusable detection is built into the core, not bolted on
4. **O(1) Indexed**: Smart internal indexing for instant random access to any grapheme
5. **Layered Architecture**: Text, modifiers, and visual elements (emoji) are separate concerns
6. **Zero Data Loss**: Perfect round-trip conversion to/from all major encodings

---

## Architecture: The Three-Layer Model

```
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

## Detailed Design of Each Component

### Component 1: The Grapheme Table (Core Data Structure)

Instead of storing text as a flat byte array (like UTF-8), UniText stores text as a **Grapheme Table** — an indexed array of grapheme entries.

```
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
- Each slot is a **fixed-size record** (e.g., 8 bytes) → **O(1) random access** by grapheme index
- The `Script` field enables **instant homograph detection** (mixed-script = suspicious)
- The `Canonical Form` is always NFC-normalized at write time → **no normalization ambiguity**
- Emoji/ZWJ sequences are stored as **references to a Visual Table**, not inline → text operations skip them cleanly

**Trade-off vs UTF-32:**
- UTF-32 is 4 bytes per code point but doesn't handle grapheme clusters, normalization, or script tagging
- UniText's Grapheme Table is 8 bytes per grapheme but gives you O(1) indexing, script metadata, normalization guarantee, AND grapheme-level atomicity — all in one

---

### Component 2: The Normalizer (Encoding Pipeline)

When text enters the UniText system (from any source: UTF-8, UTF-16, user input, clipboard, etc.), it goes through a strict pipeline:

```
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

---

### Component 3: The Security Engine (Confusable/Homograph Detector)

Built into the core, not an add-on. Uses Unicode Consortium's official confusables data (UTS #39) plus custom rules:

**Features:**
- `uniText.isSafe(text)` → returns `true/false` with a risk score
- `uniText.getConfusables("а")` → returns `["a" (Latin), "а" (Cyrillic), "ɑ" (IPA)]`
- `uniText.detectMixedScript("аpple.com")` → `WARNING: Mixed Cyrillic + Latin`
- `uniText.visuallyEqual("apple.com", "аpple.com")` → `true` (catches the attack!)

**Use cases:**
- URL/domain validation
- Username registration (prevent impersonation)
- Input sanitization for forms
- Email address verification

---

### Component 4: The Smart String API

The developer-facing API that makes text "just work":

```
UniString text = UniText.from("Hello 👨‍👩‍👧‍👦 Café");

text.length()        → 12  (human-perceived characters, NOT 19 code points)
text.charAt(6)       → "👨‍👩‍👧‍👦"  (the whole family emoji, as ONE unit)
text.reverse()       → "éfaC 👨‍👩‍👧‍👦 olleH"  (emoji stays intact, é stays intact)
text.textOnly()      → "Hello  Café"  (strip all visual/emoji content)
text.emojiOnly()     → ["👨‍👩‍👧‍👦"]  (extract visual content)
text.script()        → "Latin"  (dominant script)
text.isMixedScript() → false  (emoji doesn't count as script mixing)
text.normalize()     → guaranteed canonical (already done at creation)

// Security
text.confusablesWith("Hello") → []  (clean)
text.confusablesWith("Неllo") → ["H→Н (Cyrillic)"]  (CAUGHT!)

// Encoding export
text.toUTF8()        → standard UTF-8 bytes (for compatibility)
text.toUTF32()       → standard UTF-32 bytes
text.toASCII()       → lossy fallback with transliteration
```

---

### Component 5: The Wire Format (Compact Serialization)

For storage and transmission, we need something compact (we can't send 8-byte grapheme records over the network). UniText defines a **wire format** that is:

- **Compact**: Uses variable-length encoding (like UTF-8) for transmission
- **Self-describing**: Includes a small header indicating normalization form and version
- **Lossless**: Perfect round-trip with the internal Grapheme Table

```
┌──────────┬──────────────┬────────────────────┬─────────────────┐
│ Magic    │ Header       │ Text Payload       │ Visual Payload  │
│ "UNI\x01"│ Version+Flags│ NFC UTF-8 graphemes│ Emoji references│
│ 4 bytes  │ 4 bytes      │ Variable           │ Variable        │
└──────────┴──────────────┴────────────────────┴─────────────────┘
```

The wire format is essentially **NFC-normalized UTF-8 with a guaranteed header and separated emoji data**. This means:
- Any UTF-8 decoder can read the text payload (backward compatible!)
- But the header guarantees the normalization and the visual payload is separated
- Reading it back into UniText reconstructs the full Grapheme Table

---

## Technology & Language Choice ✅ DECIDED

**Primary Language: Rust** — Memory safe, blazing fast, excellent Unicode ecosystem (`icu4x`, `unicode-segmentation` crate), and growing open-source community. The perfect fit for security-critical text infrastructure.

### Multi-Language Strategy
1. **Core engine in Rust** — The Grapheme Table, Normalizer, Indexer, Wire Format, Security Engine
2. **CLI tool in Rust** — The `unitext` command-line tool for analysis, conversion, and security checks
3. **Python bindings (PyO3)** — So security researchers and data scientists can use it
4. **JavaScript/WASM bindings** — So it runs in browsers for the web playground
5. **C FFI header** — So developers in C/C++/Go/etc. can link against it

### Key Rust Crates We'll Build On
| Crate | Purpose |
|-------|---------|
| `unicode-segmentation` | UAX #29 grapheme cluster segmentation |
| `unicode-normalization` | NFC/NFD/NFKC/NFKD normalization |
| `icu4x` | Script detection, properties, locale data |
| `clap` | CLI argument parsing |
| `serde` | Serialization for wire format |
| `pyo3` | Python bindings |
| `wasm-bindgen` | JavaScript/WASM bindings |

---

## What Makes This Project Unique (Why It Doesn't Already Exist)

| Existing Tool | What It Does | What It DOESN'T Do |
|--------------|-------------|-------------------|
| **ICU (IBM)** | Full Unicode processing | No grapheme-first API, no security engine, massive (30MB+) |
| **libgrapheme** | Lightweight segmentation | No indexing, no security, no normalization enforcement |
| **confusable-homoglyphs** (Python) | Homograph detection | Detection only, no text processing, no normalization |
| **Swift String** | Grapheme-cluster-aware strings | Language-specific, no security, no wire format, no cross-platform |
| **Ropey (Rust)** | Efficient text editing data structure | No normalization, no security, no grapheme indexing |

**UniText combines ALL of these into one cohesive, opinionated system.** Nobody has done this.

---

## Project Structure

```
unitext/
├── Cargo.toml                    # Workspace root
├── README.md
├── LICENSE (MIT + Apache 2.0)    # Dual license (standard for Rust)
│
├── crates/
│   ├── unitext-core/             # The heart — Grapheme Table, Normalizer, Indexer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── grapheme_table.rs  # Fixed-width grapheme slot storage
│   │   │   ├── normalizer.rs     # 6-stage encoding pipeline
│   │   │   ├── segmenter.rs      # UAX #29 grapheme cluster segmentation
│   │   │   ├── classifier.rs     # Script + Category tagging
│   │   │   ├── indexer.rs        # O(1) grapheme index builder
│   │   │   └── wire_format.rs    # Compact serialization format
│   │   └── Cargo.toml
│   │
│   ├── unitext-string/           # Smart String API (UniString type)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── uni_string.rs     # The main UniString type
│   │   │   ├── ops.rs            # length, charAt, reverse, substring, etc.
│   │   │   ├── compare.rs        # Canonical comparison, visual equality
│   │   │   ├── convert.rs        # toUTF8, toUTF32, toASCII
│   │   │   └── iter.rs           # Grapheme-aware iterators
│   │   └── Cargo.toml
│   │
│   ├── unitext-security/         # Security Engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── confusables.rs    # Unicode UTS #39 confusables DB
│   │   │   ├── mixed_script.rs   # Mixed-script detection
│   │   │   ├── homograph.rs      # Visual equality & homograph detection
│   │   │   ├── risk_scorer.rs    # Threat risk scoring engine
│   │   │   └── sanitizer.rs      # Input sanitization utilities
│   │   ├── data/
│   │   │   └── confusables.txt   # Unicode Consortium confusables table
│   │   └── Cargo.toml
│   │
│   └── unitext-cli/              # CLI Analysis Tool
│       ├── src/
│       │   ├── main.rs
│       │   ├── commands/
│       │   │   ├── analyze.rs    # Deep text analysis command
│       │   │   ├── security.rs   # Security check command
│       │   │   ├── compare.rs    # Compare two strings command
│       │   │   ├── convert.rs    # Encoding conversion command
│       │   │   └── inspect.rs    # Byte-level inspection command
│       │   └── output.rs         # Pretty terminal output formatting
│       └── Cargo.toml
│
├── bindings/
│   ├── python/                   # PyO3 bindings
│   ├── wasm/                     # WASM/JS bindings
│   └── ffi/                      # C FFI header
│
├── web-playground/               # Interactive web demo
│   ├── index.html
│   ├── style.css
│   └── app.js
│
└── tests/
    ├── conformance/              # Official Unicode test suite
    ├── security/                 # Homograph attack corpus
    ├── roundtrip/                # Encoding round-trip tests
    └── benchmarks/               # Performance benchmarks
```

---

## CLI Tool Design: `unitext`

The CLI is the face of the project — the thing people interact with first. It needs to be **beautiful**, **informative**, and feel like a power tool.

### Commands

#### `unitext analyze <text>`
Dissects any text and shows everything about it:
```
$ unitext analyze "Hello 👨‍👩‍👧‍👦 Café"

╔══════════════════════════════════════════════════════╗
║  UniText Analysis Report                             ║
╠══════════════════════════════════════════════════════╣
║  Input:           "Hello 👨‍👩‍👧‍👦 Café"                     ║
║  Graphemes:       12                                 ║
║  Code Points:     19                                 ║
║  UTF-8 Bytes:     35                                 ║
║  Dominant Script: Latin                              ║
║  Mixed Script:    No                                 ║
║  Security Risk:   None ✅                            ║
╠══════════════════════════════════════════════════════╣
║  GRAPHEME BREAKDOWN                                  ║
╠══════╦═══════╦════════╦══════════╦═══════════════════╣
║ Slot ║ Char  ║ Script ║ Category ║ Code Points       ║
╠══════╬═══════╬════════╬══════════╬═══════════════════╣
║  0   ║  H    ║ Latin  ║ Letter   ║ U+0048            ║
║  1   ║  e    ║ Latin  ║ Letter   ║ U+0065            ║
║  2   ║  l    ║ Latin  ║ Letter   ║ U+006C            ║
║  3   ║  l    ║ Latin  ║ Letter   ║ U+006C            ║
║  4   ║  o    ║ Latin  ║ Letter   ║ U+006F            ║
║  5   ║  " "  ║ Common ║ Space    ║ U+0020            ║
║  6   ║  👨‍👩‍👧‍👦  ║ Emoji  ║ Visual   ║ U+1F468+ZWJ+...   ║
║  7   ║  " "  ║ Common ║ Space    ║ U+0020            ║
║  8   ║  C    ║ Latin  ║ Letter   ║ U+0043            ║
║  9   ║  a    ║ Latin  ║ Letter   ║ U+0061            ║
║  10  ║  f    ║ Latin  ║ Letter   ║ U+0066            ║
║  11  ║  é    ║ Latin  ║ Letter   ║ U+00E9 (NFC)      ║
╚══════╩═══════╩════════╩══════════╩═══════════════════╝
```

#### `unitext security <text>`
Run a full security analysis:
```
$ unitext security "аpple.com"

╔══════════════════════════════════════════════════════╗
║  ⚠️  SECURITY ALERT                                  ║
╠══════════════════════════════════════════════════════╣
║  Risk Level:    HIGH 🔴                              ║
║  Threat Type:   Homograph Attack (IDN Spoofing)      ║
╠══════════════════════════════════════════════════════╣
║  FINDINGS                                            ║
║  ┌──────┬───────┬──────────────────────────────────┐ ║
║  │ Pos  │ Char  │ Issue                            │ ║
║  ├──────┼───────┼──────────────────────────────────┤ ║
║  │  0   │  а    │ Cyrillic 'а' (U+0430)            │ ║
║  │      │       │ Confusable with Latin 'a' (U+61) │ ║
║  └──────┴───────┴──────────────────────────────────┘ ║
║                                                      ║
║  Mixed Scripts: Cyrillic + Latin (SUSPICIOUS)        ║
║  Likely Target: "apple.com" (Latin-only)             ║
║  Punycode:      xn--pple-43d.com                     ║
╚══════════════════════════════════════════════════════╝
```

#### `unitext compare <text1> <text2>`
Compare two strings at every level:
```
$ unitext compare "Café" "Cafe\u0301"

╔══════════════════════════════════════════════════════╗
║  String Comparison Report                            ║
╠══════════════════════════════════════════════════════╣
║  Byte-equal:      No  ❌  (different UTF-8 bytes)    ║
║  Codepoint-equal: No  ❌  (U+00E9 vs U+0065+U+0301) ║
║  NFC-equal:       Yes ✅  (identical after NFC)      ║
║  Grapheme-equal:  Yes ✅  (same human characters)    ║
║  Visual-equal:    Yes ✅  (look identical on screen)  ║
╚══════════════════════════════════════════════════════╝
```

#### `unitext convert <text> --to <encoding>`
Convert between encodings with full transparency:
```
$ unitext convert "Héllo" --to ascii

  Input:    "Héllo" (UTF-8, 6 bytes)
  Output:   "Hello" (ASCII, 5 bytes)
  Lossy:    Yes ⚠️  — é transliterated to e
```

#### `unitext inspect <text>`
Raw byte-level X-ray of text:
```
$ unitext inspect "é"

  Graphemes:    1
  Code Points:  1 (U+00E9 LATIN SMALL LETTER E WITH ACUTE)
  UTF-8:        C3 A9 (2 bytes)
  UTF-16:       00 E9 (2 bytes)
  UTF-32:       00 00 00 E9 (4 bytes)
  Script:       Latin
  Category:     Lowercase Letter (Ll)
  NFC Form:     U+00E9 (precomposed) ✅
  NFD Form:     U+0065 U+0301 (decomposed)
  Confusables:  None
```

---

## Project Scope & Phases

### Phase 1: Foundation & Core Engine — ~4-6 weeks
- [ ] Rust workspace setup with Cargo workspace
- [ ] `unitext-core`: Grapheme Table data structure with O(1) indexed access
- [ ] `unitext-core`: 6-stage encoding pipeline (decode → NFC → segment → classify → separate → index)
- [ ] `unitext-string`: UniString type with core operations (`length`, `char_at`, `reverse`, `substring`, `compare`)
- [ ] `unitext-string`: UTF-8 ↔ UniText round-trip conversion
- [ ] `unitext-cli`: `analyze` and `inspect` commands with beautiful terminal output
- [ ] Unit tests + official Unicode conformance tests for normalization & segmentation

### Phase 2: Security Engine — ~2-3 weeks
- [ ] `unitext-security`: Confusables database integration (Unicode UTS #39 data)
- [ ] `unitext-security`: Mixed-script detection engine
- [ ] `unitext-security`: `is_safe()`, `visually_equal()`, `get_confusables()` API
- [ ] `unitext-security`: Risk scoring engine with configurable thresholds
- [ ] `unitext-cli`: `security` and `compare` commands
- [ ] Security tests against known homograph attack corpus

### Phase 3: Wire Format & Conversion — ~2-3 weeks
- [ ] `unitext-core`: Wire format specification, encoder, and decoder
- [ ] `unitext-string`: `to_ascii()` with smart transliteration (é→e, ñ→n, ü→ue)
- [ ] `unitext-cli`: `convert` command with multi-encoding support
- [ ] Round-trip encoding tests (any valid Unicode → UniText → original = identical)

### Phase 4: Language Bindings & Web Demo — ~3-4 weeks
- [ ] Python bindings via PyO3 — `pip install unitext`
- [ ] JavaScript/WASM bindings via `wasm-bindgen` — `npm install unitext`
- [ ] C FFI header for C/C++/Go interop
- [ ] Interactive web playground (paste any text → instant analysis + security check)
- [ ] Emoji layer separation and Visual Table (advanced feature)

### Phase 5: Advanced Features & Community — ~3-4 weeks
- [ ] CJK variant-aware comparison (respecting Han Unification issues)
- [ ] Comprehensive documentation site with tutorials and API reference
- [ ] Benchmark suite (vs ICU, vs raw UTF-8 processing, vs Swift String)
- [ ] Published crate on crates.io, pip package on PyPI, npm package
- [ ] GitHub community setup (CONTRIBUTING.md, issue templates, RFC process)
- [ ] Example integrations (URL validator, username checker, form sanitizer)

---

## Decisions Made ✅

| Decision | Choice | Notes |
|----------|--------|-------|
| **Project Name** | **UniText** | Locked in for now, can be renamed later if needed |
| **Primary Language** | **Rust** | Core engine, CLI tool, and all performance-critical code |
| **Scope Priority** | **All three** | Smart String API + Security Engine + Analysis CLI Tool — all equally important |
| **Target Audience** | **Everyone** | Developers (library), Security Researchers (tool), General Public (web playground) |
| **License** | **MIT + Apache 2.0** (dual) | Standard for Rust ecosystem, maximum adoption |

---

## Verification Plan

### Automated Tests
- Unicode conformance tests (official Unicode test suite for normalization, segmentation)
- Round-trip encoding tests (any valid Unicode string → UniText → back to original encoding = identical)
- Security tests (known homograph attack corpus → all detected)
- Performance benchmarks (must beat naive approaches, competitive with ICU)

### Manual Verification  
- Test with text from 20+ writing systems (Latin, Cyrillic, Arabic, Devanagari, CJK, Thai, etc.)
- Test with adversarial inputs (zalgo text, emoji bombs, mixed-script strings)
- Test the CLI tool interactively with real-world messy text
