Setup
===

To follow this tutorial, you will need to set up some stuff on your computer.

## Substrate Prerequisites

To get started, you need to make sure your computer is set up to build Substrate.

If you are using OSX or most popular Linux distros, you can do that with a simple one-liner:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

## Installing Substrate

You need to use a Substrate node with the built-in Contracts module.

You can install the default Substrate template by running the next command.

```bash
curl https://getsubstrate.io -sSf | bash
```

If you are using another operating system, like Windows, follow the [installation instructions](https://substrate.dev/docs/en/getting-started/installing-substrate#windows) on the Substrate Developer Hub.


## ink! CLI

The final tool we will be installing is the ink! command line utility which will make setting up Substrate smart contract projects easier.

You can install the utility using Cargo with:

```bash
cargo install --force --git https://github.com/paritytech/ink cargo-contract
```

You can then use `cargo contract --help` to start exploring the commands made available to you.  
> **Note**: The ink! CLI is under heavy development and most of its commands are not implemented, yet!

