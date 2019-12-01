# adventofcode19

My solutions for this year's [Advent of Code](https://adventofcode.com/2019), in [Rust](https://www.rust-lang.org/)

Run a given day's solution with

```
cargo run --release --bin day-01
```

Optionally, you can also run the solutions with WebAssembly!

```
# Setup: install wasmtime
git clone --recurse-submodules https://github.com/CraneStation/wasmtime.git;
cd wasmtime;
cargo build --release;
cp target/release/wasmtime ~/.bin/;
# ^ Or another directory that is in your executable $PATH
cd ..;
```

```
cargo build --target wasm32-wasi --release
wasmtime target/wasm32-wasi/release/day-01.wasm
```