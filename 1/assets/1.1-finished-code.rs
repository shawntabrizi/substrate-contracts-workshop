#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod incrementer {
    #[ink(storage)]
    struct Incrementer {
        // Storage Declaration
    }

    impl Incrementer {
        #[ink(constructor)]
        fn new(&mut self, init_value: i32) {
            // Contract Constructor
        }

        #[ink(message)]
        fn get(&self) {
            // Contract Message
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            // Test Your Contract
        }
    }
}