#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod incrementer {
    use ink_core::storage;

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
        fn default_works() {
            // Test Your Contract
        }
    }
}