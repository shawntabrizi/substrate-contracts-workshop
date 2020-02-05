Storing a Mapping
===

Let's now extend our Incrementer to not only manage one number, but to manage one number per user!

## Storage HashMap

In addition to `storage::Value`, ink! also supports a `storage::HashMap` which allows you to store items in a key-value mapping.

Here is an example of a mapping from user to a number:

```rust
#[ink(storage)]
struct MyContract {
    // Store a mapping from AccountIds to a u32
    my_number_map: storage::HashMap<AccountId, u32>,
}
```

This means that for a given key, you can store a unique instance of a value type. In this case, each "user" gets their own number, and we can build logic so that only they can modify their own number.

## Storage HashMap API

You can find the full HashMap API in the [core/storage/collections/hashmap](https://github.com/paritytech/ink/blob/master/core/src/storage/collections/hash_map/impls.rs) part of ink!.

Here are some of the most common functions you might use:

```rust
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated,
    /// and the old value is returned.
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {/* --snip-- */}

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> {/* --snip-- */}

    /// Returns an immutable reference to the value corresponding to the key.
    pub fn get<Q>(&self, key: &Q) -> Option<&V> {/* --snip-- */}

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V> {/* --snip-- */}

    /// Mutates the value associated with the key if any.
    /// Returns a reference to the mutated element
    pub fn mutate_with<Q, F>(&mut self, key: &Q, f: F) -> Option<&V> {/* --snip-- */}
    
```

## Initializing a HashMap

As mentioned a number of times throughout this tutorial, not initializing storage before you use it is a common error that can break your smart contract. For each key in a storage value, the value needs to be set before you can use it. To do this, we will create a private function which handles when the value is set and when it is not, and make sure we never work with uninitialized storage.

So given `my_number_map`, imagine we wanted the default value for any given key to be `0`. We can build a function like this:

```rust
#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod mycontract {
    
    #[ink(storage)]
    struct MyContract {
        // Store a mapping from AccountIds to a u32
        my_number_map: storage::HashMap<AccountId, u32>,
    }

    impl MyContract {
        /// Private function.
        /// Returns the number for an AccountId or 0 if it is not set.
        fn my_number_or_zero(&self, of: &AccountId) -> u32 {
            let balance = self.my_number_map.get(of).unwrap_or(&0);
            *balance
        }
    }
}
```

Here we see that after we `get` the value from `my_number_map` we call `unwrap_or` which will either `unwrap` the value stored in storage, _or_ if there is no value, return some known value. Then, when building functions that interact with this HashMap, you need to always remember to call this function rather than getting the value directly from storage.

Here is an example:

```rust
#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod mycontract {
    
    #[ink(storage)]
    struct MyContract {
        // Store a mapping from AccountIds to a u32
        my_number_map: storage::HashMap<AccountId, u32>,
    }

    impl MyContract {
        // Get the value for a given AccountId
        #[ink(message)]
        fn get(&self, of: AccountId) -> u32 {
            let value = self.my_number_or_zero(&of);
            value
        }

        // Get the value for the calling AccountId
        #[ink(message)]
        fn get_my_number(&self) -> u32 {
            let caller = self.env().caller();
            let value = self.my_number_or_zero(&caller);
            value
        }

        // Returns the number for an AccountId or 0 if it is not set.
        fn my_number_or_zero(&self, of: &AccountId) -> u32 {
            let value = self.my_number_map.get(of).unwrap_or(&0);
            *value
        }
    }
}
```

## Contract Caller

As you might have noticed in the example above, we use a special function called `self.env().caller()`. This function is available throughout the contract logic and will always return to you the contract caller.

> **NOTE:** The contract caller is not the same as the origin caller. If a user triggers a contract which then calls a subsequent contract, the `self.env().caller()` in the second contract will be the address of the first contract, not the original user.

`self.env().caller()` can be used a number of different ways. In the examples above, we are basically creating an "access control" layer which allows a user to modify their own value, but no one else. You can also do things like define a contract owner during contract deployment:

```rust
#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod mycontract {
    
    #[ink(storage)]
    struct MyContract {
        // Store a contract owner
        owner: storage::Value<AccountId>,
    }

    impl MyContract {
        #[ink(constructor)]
        fn new(&mut self, init_value: i32) {
            self.owner.set(self.env().caller());
        }
        /* --snip-- */
    }
}
```

Then you can write permissioned functions which checks that the current caller is the owner of the contract.

## Your Turn!

Follow the `ACTION`s in the template code to introduce a storage map to your contract.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.5-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.5-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/1.4-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->