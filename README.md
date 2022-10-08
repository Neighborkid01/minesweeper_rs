# Minesweeper in Rust

This is a rust program that uses Yew (a component framework similar to React) to create a minesweeper game that compiles to web assembly and runs in the browser.

## Getting Started
1. Have Rust [installed](https://www.rust-lang.org/tools/install)
2. Add WebAssembly target
    - `rustup target add wasm32-unknown-unknown`
3. Install Trunk and wasm-bindgen-cli
    - `cargo install --locked trunk`
    - `cargo install wasm-bindgen-cli`
4. Run `trunk serve` from the project's root directory to spin up a server on port 8080 or add the `--release` flag to compile with optimizations
5. That's it!