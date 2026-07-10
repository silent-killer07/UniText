# Contributing to UniText

First off, thank you for considering contributing to UniText! It's people like you that make UniText such a great tool.

## Development Environment Setup

1. **Rust Toolchain**: Install [rustup](https://rustup.rs/).
2. **WASM**: Install `wasm-pack` via `cargo install wasm-pack` (if working on `unitext-wasm`).
3. **Python**: Create a virtual environment and `pip install maturin` (if working on `unitext-python`).

## Workflow

1. Fork the repo and create your branch from `main`.
2. Make your changes in the appropriate workspace crate.
3. Run `cargo fmt` to format your code.
4. Run `cargo clippy` to catch common mistakes.
5. Ensure all tests pass: `cargo test --workspace`.
6. Run benchmarks if you modified performance-critical code: `cargo bench`.

## Pull Request Process

1. Ensure your PR description clearly describes the problem and solution.
2. If it's a new feature, include a test case.
3. Link any relevant issues.

## Code Style

We follow standard Rust formatting. CI will fail if `cargo fmt` has not been run. Please keep your code clean, modular, and well-documented.
