# adventofcode19

My solutions for this year's [Advent of Code](https://adventofcode.com/2019), in [Rust](https://www.rust-lang.org/)

Run a given day's solution with

```
cargo run --release --bin day-01
```

Optionally, you can also run the solutions with WebAssembly!

```
# Setup: install wasmtime
curl https://wasmtime.dev/install.sh -sSf | bash
```

```
cargo build --target wasm32-wasi --release
wasmtime target/wasm32-wasi/release/day-01.wasm
```