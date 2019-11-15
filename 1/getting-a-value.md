Getting a Value
===

Now that we have created and initialized a storage value, we are going to start to interact with it!

## Contract Functions

As you can see in the contract template, all of your contract functions are part of your contract module.

```rust
impl MyContract {
    // Public and Private functions can go here
}
```

### Public and Private Functions

In Rust, you can make as many implementations as you want. As a stylistic choice, we recommend breaking up your implementation definitions for your private and public functions:

```rust
impl MyContract {
    /// Public function
    #[ink(message)]
    fn my_public_function(&self) {
        /* --snip-- */
    }

    /// Private function
    fn my_private_function(&self) {
        /* --snip-- */
    }

    /* --snip-- */
}
```

You can also choose to split things up however is most clear for your project.

Note that all public functions must use the `#[ink(message)]` attribute.

## Storage Value API

Without going into so much detail, storage values are a part of the underlying ink! core layer. In the background, they use a more primitive `cell` type which holds an `Option<T>`. When we try to get the value from storage, we `unwrap` the value, which is why it panics if it is not initialized!

From [core/storage/value.rs](https://github.com/paritytech/ink/blob/master/core/src/storage/value.rs):

```rust
impl<T> Value<T>
where
    T: scale::Codec,
{
    /// Returns an immutable reference to the wrapped value.
    pub fn get(&self) -> &T {
        self.cell.get().unwrap()
    }

    /// Returns a mutable reference to the wrapped value.
    pub fn get_mut(&mut self) -> &mut T {
        self.cell.get_mut().unwrap()
    }

    /// Sets the wrapped value to the given value.
    pub fn set(&mut self, val: T) {
        self.cell.set(val);
    }
}
```

In that same file, you can find the other APIs exposed by storage values, however these three are the most commonly used.

## Getting a Value

We already showed you how to use `set` when we initialized the storage value. Getting the value is just as simple:

```rust
impl MyContract {
    #[ink(message)]
    fn my_getter(&self) -> u32 {
        let number = *self.my_number.get();
        number
    }
}
```

You should take notice that the `get` API returns a _reference_ to the value, so to actually get the value you need to dereference it with an asterisks (`*`). In Rust, if the last expression in a function does not have a semicolon, then it will be the return value.

You can also drop `.get()` to implicitly get the value:

```rust
impl MyContract {
    #[ink(message)]
    fn my_getter(&self) -> u32 {
        *self.my_number
    }
}
```

## Your Turn!

Follow the `ACTION`s on the code template provided.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->