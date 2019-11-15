Contract Template
===

Let's take a look at a high level what is available to you when developing a smart contract using the ink!.

## ink!

ink! is an [eDSL](https://wiki.haskell.org/Embedded_domain_specific_language) to write WebAssembly based smart contracts in the Rust programming language.

ink! is just standard Rust in a well defined "contract format" with specialized `#[ink(...)]` attribute macros. These attribute macros tell ink! what the different parts of your Rust smart contract represent, and ultimately allows ink! to do all the magic needed to create Substrate compatible Wasm bytecodes!

## Your Turn!

We are going to start a new project for the Incrementer contract we will build in this chapter.

So go into your working directory and run:

```bash
cargo contract new incrementer
```

Just like before, this will create a new project folder named `incrementer` which we will use for the rest of this chapter.

```bash
cd incrementer/
```

In the `lib.rs` file, replace the "Flipper" contract source code with the template code provided here.

Quickly check that it compiles and the trivial test passes with:

```bash
cargo +nightly test
```

Also check that you can build the Wasm file by running:

```bash
cargo contract build
```

If everything looks good, then we are ready to start programming!

<!-- tabs:start -->

#### ** Solution **

[embedded-code-final](./assets/1.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->