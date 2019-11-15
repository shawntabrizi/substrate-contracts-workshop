#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod incrementer {
    #[ink(storage)]
    struct Incrementer {
        // ACTION: Create a `storage::Value` called `value` which holds a `i32`
    }

    impl Incrementer {
        #[ink(constructor)]
        fn new(&mut self, init_value: i32) {
            // ACTION: `set` the initial value of `value` with `init_value`
        }

        // ACTION: Create a second constructor function named `default`.
        //         It has no input, and calls the `new` constructor with
        //         an `init_value` of 0.

        #[ink(message)]
        fn get(&self) {
            // Contract Message
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let contract = Incrementer::default();
        }
    }
}