# CJK Variant Architecture in UniText

Han Unification (the process by which the Unicode Consortium merged visually similar or historically related characters from Chinese, Japanese, and Korean into shared code points) introduces significant challenges for security and visual equality checking.

For example, a character might look different depending on the font or locale, and two distinct code points might look identical to a user depending on context. The Unihan database provides rich mapping data for these variants.

## Current Implementation (Lightweight)

Currently, UniText avoids embedding the massive (~30MB) Unihan database directly into the binary to maintain a small footprint suitable for WebAssembly and lightweight CLI use cases. 

Instead, our security engine relies on **UTS #39 Confusables** (embedded in `unitext-security`) which already contains the most critical cross-script and intra-script confusable mappings, including many common CJK confusables that could be used in homograph attacks.

## Future Architecture: `kZVariant` Integration

For enterprise applications requiring strict CJK variant resolution, UniText is designed to support future expansion via a feature-flagged or lazily-loaded variant table.

### 1. Data Structure

A future `CjkVariantTable` would parse the Unihan `kZVariant`, `kSemanticVariant`, and `kSimplifiedVariant` properties.

```rust
// Proposed structure for future implementation
pub struct CjkVariantTable {
    // Maps a base code point to a list of variant code points
    pub variants: HashMap<u32, Vec<u32>>,
}
```

### 2. Integration Points

The integration would happen primarily in `unitext-security` inside the `visually_equal` function:

```rust
// Proposed flow inside visually_equal
pub fn visually_equal(a: &str, b: &str) -> bool {
    // 1. Standard canonical & UTS #39 checks (Existing)
    if is_uts39_confusable(a, b) { return true; }
    
    // 2. CJK Variant Check (Future)
    #[cfg(feature = "cjk-variants")]
    if is_cjk_variant(a, b) { return true; }

    false
}
```

### 3. Loading Strategy

To prevent binary bloat, the `CjkVariantTable` should be generated at build time (similar to our UTS #39 script) but only included if a Cargo feature like `cjk-variants` is enabled:

```toml
[features]
default = []
cjk-variants = [] # Enables the 30MB Unihan mapping
```

Alternatively, the table could be provided at runtime via a custom API `UniText::load_cjk_data(path)`.

This architecture ensures UniText remains fast and small for 99% of use cases, while offering an expansion path for specialized CJK processing.
