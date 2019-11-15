#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    #[ink(storage)]
    struct Erc20 {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// The balance of each user.
        balances: storage::HashMap<AccountId, Balance>,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            // ACTION: `set` the total supply to `init_value`
            // ACTION: `insert` the `init_value` as the `caller` balance
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            // ACTION: Return the total supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            // ACTION: Return the balance of `owner`
            //   HINT: Use `balance_of_or_zero` to get the `owner` balance
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            // ACTION: `get` the balance of `owner`, then `unwrap_or` fallback to 0
            // ACTION: Return the balance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 0);
        }
    }
}