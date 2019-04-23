Safe Math
===

Talk about Rust default overflow settings
- Release vs Test

Talk about `.cargo/config`

```rust
[target.wasm32-unknown-unknown]
rustflags = [
	"-C", "overflow-checks=on",
	"-C", "link-args=-z stack-size=65536 --import-memory"
]
```

Talk about `overflow-checks=on`

Show an example of a situation which should panic automatically.