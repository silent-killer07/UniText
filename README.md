# UniText 🛡️📝

> A Next-Generation Text Encoding Abstraction System.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://github.com/silent-killer07/UniText/actions/workflows/rust.yml/badge.svg)](https://github.com/silent-killer07/UniText/actions/workflows/rust.yml)

Unicode is great, but it has flaws: normalization chaos, homograph attacks (e.g., Cyrillic `а` vs Latin `a`), and invisible grapheme boundaries. **UniText** solves this by providing a grapheme-first, security-native text engine.

## 🚀 Features
- **Grapheme-First**: `UniString::length("👨‍👩‍👧‍👦") == 1`
- **Security-Native**: Built-in detection for homograph attacks and mixed-script impersonation.
- **Canonical-Only**: All text is automatically NFC-normalized.
- **Cross-Language**: Rust core with Python, WASM, and C FFI bindings.

## 📦 Installation

**Rust:**
```toml
[dependencies]
unitext-core = "0.1"
unitext-string = "0.1"
unitext-security = "0.1"
```

**Python:**
```bash
pip install unitext
```

**JavaScript/WASM:**
```bash
npm install unitext-wasm
```

## 🛠️ Usage (Rust)

```rust
use unitext_string::UniString;

fn main() {
    let text = UniString::new("Hello 👨‍👩‍👧‍👦 Café");
    
    println!("Graphemes: {}", text.length()); // 12
    println!("Is Safe? {}", text.is_safe());  // true

    // Catch Homograph Attacks!
    let safe = "apple.com";
    let unsafe_str = "аpple.com"; // Cyrillic 'a'
    
    let is_attack = UniString::visually_equal(safe, unsafe_str);
    println!("Caught homograph attack? {}", is_attack); // true
}
```

## 💻 CLI Tool Usage

UniText comes with a powerful CLI for text analysis and security scanning.

```bash
# Analyze text structure and graphemes
cargo run -p unitext-cli -- analyze "Hello 👨‍👩‍👧‍👦 Café"

# Run a security scan for homograph attacks
cargo run -p unitext-cli -- security "аpple.com"
```

## 🌐 Web Playground

Try UniText directly in your browser! The web playground uses our WASM bindings to run the core engine entirely client-side.

```bash
cd web-playground
npx http-server . -p 8000
```
Then open `http://localhost:8000` in your browser.

## 📚 Documentation
- [Architecture & Design](docs/architecture.md)
- [CJK Variant Strategy](docs/cjk-variants.md)

## 🤝 Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to set up your environment, run tests, and submit PRs.

## ⚖️ License
Dual-licensed under MIT or Apache 2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
