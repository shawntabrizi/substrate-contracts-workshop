#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod incrementer {
    #[ink(storage)]
    struct Incrementer {
        value: storage::Value<i32>,
        my_value: storage::HashMap<AccountId, u64>,
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
            self.my_value_or_zero(&self.env().caller())
        }

        #[ink(message)]
        fn inc_mine(&mut self, by: u64) {
            let caller = self.env().caller();
            let my_value = self.my_value_or_zero(&caller);
            self.my_value.insert(caller, my_value + by);
        }

        fn my_value_or_zero(&self, of: &AccountId) -> u64 {
            *self.my_value.get(of).unwrap_or(&0)
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
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 15);
            
        }
    }
}