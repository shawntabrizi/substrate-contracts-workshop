Setup
===

To follow this tutorial, you will need to set up some stuff on your computer.

## Substrate Prerequisites

To get started, you need to make sure your computer is set up to build Substrate.

If you are using __OSX__ or most popular __Linux__ distros, you can do it by running:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

```
rustup target add wasm32-unknown-unknown --toolchain stable
```
If you are using another operating system, like __Windows__, follow the [installation instructions](https://substrate.dev/docs/en/getting-started/installing-substrate#windows) on the Substrate Developer Hub.

## Installing Substrate

We need to use a Substrate node with the built-in Contracts module. For this workshop we'll use the pre-designed Substrate Node.

```bash
curl https://getsubstrate.io -sSf | bash
```


## ink! CLI

The final tool we will be installing is the ink! command line utility which will make setting up Substrate smart contract projects easier.

You can install the utility using Cargo with:

```bash
cargo install --force --git https://github.com/paritytech/ink cargo-contract
```

You can then use `cargo contract --help` to start exploring the commands made available to you.  
> **Note:** The ink! CLI is under heavy development and some of its commands are not implemented, yet!

