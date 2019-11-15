Building Your Contract
===

Run the following command to compile your smart contract:

```bash
cargo contract build
```

This special command will turn your ink! project into a Wasm binary which you can deploy to your chain. If all goes well, you should see a `target` folder which contains this `.wasm` file.

```
target
└── flipper.wasm
```

## Contract Metadata

By running the next command we'll generate the contract metadata (a.k.a. the contract ABI):

``` bash
cargo contract generate-abi
```

You should have a new JSON file (`abi.json`) in the same target directory: 

``` bash
target
├── flipper.wasm
└── abi.json
```

Let's take a look at the structure inside:

``` JSON
{
  "registry": {
    "strings": [...],
    "types": [...]
  },
  "storage": {...},
  "contract": {
    "name": ...,
    "constructors": [...],
    "messages": [...],
    "events": [],
    "docs": []
  }
}
```

You can see that this file describes all the interfaces that can be used to interact with your contract.

* Registry provides the **strings** and cutom **types** used throughout the rest of the JSON.
* Storage defines all the **storage** items managed by your contract and how to ultimately access them.
* Contract stores information about the callable functions like  **constructors** and **messages** a user can call to interact with your contract. It also has helpful information like the **events** that are emitted by the contract or any **docs**.

If you look close at the constructors and messages, you will also notice a `selector` which is a 4-byte hash of the function name and is used to route your contract calls to the correct functions.

Polkadot-JS Apps uses this file to generate a friendly interface for deploying and interacting with your contract. :)

In the next section we will start a Substrate node and configure the Polkadot-JS Apps UI to interact with it.

---

**Learn More**

ink! provides a built-in overflow protection enabled on our `Cargo.toml` file. It is __recommended__ to keep it enabled as a security mechanism.
```
[profile.release]
panic = "abort"           <-- Panics shall be treated as aborts: reduces binary size
lto = true                <-- enable link-time-optimization: more efficient codegen
opt-level = "z"           <-- Optimize for small binary output
overflow-checks = true    <-- Arithmetic overflow protection
```
After running all Rust and LLVM optimizations, we apply extra steps to create a more efficient WebAssembly [`wasm`] file.

WebAssembly modules can use two parameters to specify how much memory it wants:

1. Initial Size - the size of the memory when it is first imported.
2. Maximum Size - the maximum size the memory can grow to.

It is encoded like:

```
(import "env" "memory" (memory <initial> <maximum>))
```

If Maximum Size is absent then it is implicitly set to 4GB.

---
