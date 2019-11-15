#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod incrementer {
    #[ink(storage)]
    struct Incrementer {
        value: storage::Value<i32>,
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }
    }
}