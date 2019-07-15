Building Your Contract
===

The ink! CLI also generates a build script called `build.sh`:

```bash
#!/bin/bash
set -e

PROJNAME=flipper

# cargo clean
# rm Cargo.lock

CARGO_INCREMENTAL=0 &&
cargo build --release --features generate-api-description --target=wasm32-unknown-unknown --verbose
wasm2wat -o target/$PROJNAME.wat target/wasm32-unknown-unknown/release/$PROJNAME.wasm
cat target/$PROJNAME.wat | sed "s/(import \"env\" \"memory\" (memory (;0;) 2))/(import \"env\" \"memory\" (memory (;0;) 2 16))/" > target/$PROJNAME-fixed.wat
wat2wasm -o target/$PROJNAME.wasm target/$PROJNAME-fixed.wat
wasm-prune --exports call,deploy target/$PROJNAME.wasm target/$PROJNAME-pruned.wasm
```

This file will be used to compile your contract source code to WASM. You can see that it depends on the Wasm utilities we installed earlier.

To compile the smart contract, we need to run it:

```bash
./build.sh
```

If all goes well, you should see a `target` folder being created with 5 relevant files corresponding to the steps in the script:

```
target
├── flipper-fixed.wat
├── flipper-pruned.wasm
├── flipper.wasm
└── flipper.wat
```

The final, optimized `flipper-pruned.wasm` file is what we will actually deploy to our Substrate chain.

## Contract ABI

You will also notice a JSON file which is generated during the build script:

```
Flipper.json
```

This is your contract's Application Binary Interface (ABI). Let's take a look inside:

```json
{
    "name": "Flipper",
    "deploy": {
        "args": []
    },
    "messages": [
        {
            "name": "flip",
            "selector": 970692492,
            "mutates": true,
            "args": [],
            "return_type": null
        },
        {
            "name": "get",
            "selector": 4266279973,
            "mutates": false,
            "args": [],
            "return_type": "bool"
        }
    ]
}
```

You can see that this file describes the interface that can be used to interact with your contract.

If there are any deployment variables needed when instantiating a new contract, those will be defined in the `deploy` section. All the public functions your contract exposes can be found in `messages` along with its function name, function parameters, return type, and whether the function is read-only.

There is also a `selector` which is a hash of the function name and is used to route your contract calls to the correct function.

The Polkadot UI uses this file to generate a friendly interface for deploying and interacting with your contract. :)

In the next section we will configure the Polkadot UI.

---

**Learn More**

One line in the build script we should call out is:

```bash
cat target/$PROJNAME.wat | sed "s/(import \"env\" \"memory\" (memory (;0;) 2))/(import \"env\" \"memory\" (memory (;0;) 2 16))/" > target/$PROJNAME-fixed.wat &&
```

TL;DR, this line is adding a maximum size to the Wasm memory declaration, which by default is not included.

WebAssembly modules can use two parameters to specify how much memory it wants:

1. Initial Size - the size of the memory when it is first imported.
2. Maximum Size - the maximum size the memory can grow to.

It is encoded like:

```
(import "env" "memory" (memory <initial> <maximum>))
```

If Maximum Size is absent then it is implicitly set to 4GB.

---