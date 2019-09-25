Building Your Contract
===

To compile the smart contract, we need to run it:

```bash
cargo contract build
```

If all goes well, you should see a `target` folder being created with 5 relevant files corresponding to the steps in the script:

```
target
└── flipper.wasm
```

The final, optimized `flipper.wasm` file is what we will actually deploy to our Substrate chain.

## Contract ABI
By running the next command we'll generate the Application Binary Interface (ABI):
``` bash
cargo +nightly build --features ink-generate-abi
```

You should have a new JSON file (`old_abi.json`) in the same target directory. This is your contract's ABI. 

``` bash
target
├── flipper.wat
└── old_abi.json
```

Let's take a look inside:

``` JSON
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