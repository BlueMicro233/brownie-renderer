# brownie-renderer-rust

A near one-to-one Rust port of the original C++ Brownie path tracer.

## Build

```bash
cd rust
cargo build --release
```

## Run

From repository root:

```bash
cargo run --release --manifest-path rust/Cargo.toml
```

Then input `spp` in terminal. The output image is written to `binary.ppm`.
