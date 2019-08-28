Incrementing My Value
===

The final step in our Incrementer contract is to allow each use to update increment their own value.

## Modifying a HashMap

Making changes to the value of a HashMap is just as sensitive as getting the value. If you try to modify some value before it has been initialized, your contract will panic! (Have you been keeping count the number of times we say this?)

But have no fear, we can continue to use the `my_number_or_zero` function we created to protect us from these situations!

```rust
impl MyContract {
    ...

    // Set the value for the calling AccountId
    pub(external) fn set_my_number(&mut self, value: u32) {
        let caller = env.caller();
        self.my_number_map.insert(caller, value);
    }

    // Add a value to the existing value for the calling AccountId
    pub(external) fn add_my_number(&mut self, value: u32) {
        let caller = env.caller();
        let my_number = self.my_number_or_zero(&caller);
        self.my_number_map.insert(caller, my_number + value);
    }
}

impl MyContract {
    /// Returns the number for an AccountId or 0 if it is not set.
    fn my_number_or_zero(&self, of: &AccountId) -> u32 {
        let value = self.my_number_map.get(of).unwrap_or(&0);
        *value
    }
}
```

Here we have written two kinds of functions which modify a HashMap. One which simply inserts the value directly into storage, with no need to read the value first, and the other which modifies the existing value. Note how we can always `insert` the value without worry, as that initialized the value in storage, but before you can get or modify anything, we need to call `my_number_or_zero` to make sure we are working with a real value.

## Feel the Pain (Optional)

TODO: Add content which walks a user through reading/modifying uninitialized storage.

## Your Turn!

Follow the `ACTION`s to finish your Incrementer smart contract.

Remember to run `cargo test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.6-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.6-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->