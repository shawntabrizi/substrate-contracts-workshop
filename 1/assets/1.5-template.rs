#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod incrementer {
    use ink_core::storage;

    #[ink(storage)]
    struct Incrementer {
        value: storage::Value<i32>,
        // ACTION: Add a `HashMap` from `AccountId` to `u64` named `my_value`
    }

    impl Incrementer {
        #[ink(constructor)]
        fn new(&mut self, init_value: i32) {
            self.value.set(init_value);
        }

        #[ink(constructor)]
        fn default(&mut self) {
            self.new(0)
        }

        #[ink(message)]
        fn get(&self) -> i32 {
            *self.value
        }

        #[ink(message)]
        fn inc(&mut self, by: i32) {
            *self.value += by;
        }

        #[ink(message)]
        fn get_mine(&self) -> u64 {
            // ACTION: Get `my_value` using `my_value_or_zero` on `&self.env().caller()`
            // ACTION: Return `my_value`
        }

        fn my_value_or_zero(&self, of: &AccountId) -> u64 {
            // ACTION: `get` and return the value of `of` and `unwrap_or` return 0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[test]
        fn my_value_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }
    }
}