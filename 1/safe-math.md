Safe Math
===

ink! provides a built-in overflow protection enabled on our `Cargo.toml` file. It is __recommended__ to keep it enabled as a security mechanism.
```
[profile.release]
panic = "abort"           <-- Panics shall be treated as aborts: reduces binary size
lto = true                <-- enable link-time-optimization: more efficient codegen
opt-level = "z"           <-- Optimize for small binary output
overflow-checks = true    <-- Arithmetic overflow protection
```