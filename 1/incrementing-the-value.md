Incrementing the Value
===

It's time to let our users modify storage!

## Mutable and Immutable Functions

You may have noticed that the function templates included `self` as the first parameter of the contract functions. It is through `self` that you gain access to all your contract functions and storage items.

If you are simply _reading_ from the contract storage, you only need to pass `&self`. But if you want to _modify_ storage items, you will need to explicitly mark it as mutable, `&mut self`.

```rust
impl MyContract {
    #[ink(message)]
    fn my_getter(&self) -> u32 {
        *self.my_number
    } 

    #[ink(message)]
    fn my_setter(&mut self, new_value: u32) {
        self.my_number.set(new_value);
    }
}
```

## Modifying a Storage Value

You can always update the value of a storage item by calling `set` again.

However, if you know the value is already set, then you can modify the value in a more ergonomic way:

```rust
impl MyContract {
    #[ink(message)]
    fn my_setter(&mut self, new_value: u32) {
        self.my_number.set(new_value);
    }

    #[ink(message)]
    fn my_adder(&mut self, add_value: u32) {
        self.my_number += add_value;
    }
}
```

However, if the value is not initialized, your contract will compile fine, but will panic during contract execution! **We really cannot understate how easy it is to make mistakes this way.**

## Your Turn

Follow the `ACTION`s in the template code.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.4-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/1.3-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
