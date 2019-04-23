Contract Template
===

We are going to start a new project for the Incrementer contract we will build in this chapter.

## Your Turn!

So go into your working directory and run:

```bash
cargo contract new incrementer
```

Just like before, this will create a new project folder named `incrementer` which we will use for the rest of this chapter.

In the `src/lib.rs` file, replace the "Flipper" contract source code with the template code provided here.

Quickly check that it compiles and the trivial test passes with:

```bash
cargo test --features test-env
```

Also check that you can build the Wasm file by running `build.sh`. Remember, you will need to run `chmod +x build.sh` to make the script executable.

If everything looks good, then we are ready to start programming!

<!-- tabs:start -->

#### ** Solution **

[embedded-code-final](./assets/1.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->