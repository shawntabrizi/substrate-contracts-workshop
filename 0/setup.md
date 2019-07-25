Setup
===

To follow this tutorial, you will need to set up some stuff on your computer.

> **Note:** For the purposes of stability, we will be using Substrate v1.0 for this tutorial. In addition to the steps below, please download the [`substrate-package` for contract development](https://github.com/shawntabrizi/substrate-package/tree/contract).
>
> This will include a `substrate-node-template` with the Contract module included in your node. It also includes an `ink-contract-template` where you can start to build your smart contract.
>
> Please use this package for the remainder of the tutorial.

## Substrate Prerequisites

To get started, you need to make sure your computer is set up to build Substrate.

If you are using OSX or most popular Linux distros, you can do that with a simple one-liner:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

If you are using another operating system, like Windows, follow the [installation instructions](https://substrate.dev/docs/en/getting-started/installing-substrate#windows) on the Substrate Developer Hub.

## Wasm Build Environment

Next you need to set up the Wasm build environment within Rust:

```bash
rustup update nightly
rustup update stable

rustup target add wasm32-unknown-unknown --toolchain nightly
```

While ink! is pinned to a specific nightly version of the Rust compiler you will need to explicitly install that toolchain. At the time of writing this, it is pinned to `nightly-2019-05-21`, but you can look at your `rust-toolchain` if you run into any issues:

```bash
rustup install nightly-2019-05-21
rustup target add wasm32-unknown-unknown --toolchain nightly-2019-05-21
```

## Wasm Utilities

Smart contracts in Substrate are compiled to WebAssembly (Wasm). To manipulate these files for use on Substrate, you will need to install some Wasm utilities:

* [Wabt](https://github.com/WebAssembly/wabt)
* [Parity wasm-utils](https://github.com/paritytech/wasm-utils)

Depending on your operating system, the installation instruction may be different:

**Mac OS**:

```bash
brew install wabt

cargo install pwasm-utils-cli --bin wasm-prune --force
```

**Arch Linux**:

```bash
sudo pacman -Syu wabt

cargo install pwasm-utils-cli --bin wasm-prune --force
```

**Ubuntu/Debian**:

```bash
sudo apt install -y curl jq tar

curl https://raw.githubusercontent.com/substrate-developer-hub/substrate-contracts-workshop/master/scripts/install-wasm-tools.sh -sSf |bash -s

cargo install pwasm-utils-cli --bin wasm-prune --force
```

We will be using `wasm2wat` (wabt), `wat2wasm` (wabt), and `wasm-prune` (wasm-utils) later in the guide.

> ** The Ubuntu/Debian script will install 2 external binaries (wasm2wat, wat2wasm) into the ~/.cargo/bin directory.

## ink! CLI

The final tool we will be installing is the ink! command line utility which will make setting up Substrate smart contract projects easier.

You can install the utility using Cargo with:

```bash
cargo install --force --git https://github.com/paritytech/ink cargo-contract
```

You can then use `cargo contract --help` to start exploring the commands made available to you.  
> **Note**: The ink! CLI is under heavy development and most of its commands are not implemented, yet!
