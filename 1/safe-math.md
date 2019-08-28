Safe Math
===

Talk about Rust default overflow settings
- Release vs Test

Talk about `.cargo/config`

```rust
[target.wasm32-unknown-unknown]
rustflags = [
        "-C", "link-args=-z stack-size=65536 --import-memory"
]
```