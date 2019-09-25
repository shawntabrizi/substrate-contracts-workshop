Contract Template
===

Let's take a look at a high level what is available to you when developing a smart contract using the ink!.

## ink!

ink! is an [eDSL](https://wiki.haskell.org/Embedded_domain_specific_language) to write WebAssembly based smart contracts in the Rust programming language.

ink! exists at 3 different layers:

* Core: The core utilities used to write smart contracts.
* Model: Medium-level abstractions to write smart contracts heavily inspired by [Fleetwood](https://github.com/paritytech/fleetwood).
* Language (Lang): The actual eDSL based on ink! Core and ink! Model to provide a user friendly interface to writing smart contract code.

The Language layer of ink! relies on a single, heavy macro called `contract!`. At compile time, this macro expands to generate code at the Model and Core level. For the purposes of this guide, we will be focusing on the Language layer of ink! where we expect most contract development to take place. Take a quick look over the ink! contract template provided here.

## Your Turn!

We are going to start a new project for the Incrementer contract we will build in this chapter.

So go into your working directory and run:

```bash
cargo contract new incrementer
```

Just like before, this will create a new project folder named `incrementer` which we will use for the rest of this chapter.

In the `src/lib.rs` file, replace the "Flipper" contract source code with the template code provided here.

Quickly check that it compiles and the trivial test passes with:

```bash
cargo +nightly test
```

Also check that you can build the Wasm file by running `cargo contract build`.

If everything looks good, then we are ready to start programming!

<!-- tabs:start -->

#### ** Solution **

[embedded-code-final](./assets/1.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->