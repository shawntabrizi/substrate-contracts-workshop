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
        /// Approval spender on behalf of the message's sender.
        allowances: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            // ACTION: Get the `self.env().caller()` and store it as the `owner`
            // ACTION: Insert the new allowance into the `allowances` HashMap
            //   HINT: The key tuple is `(owner, spender)`
            // ACTION: `emit` the `Approval` event you created using these values
            // ACTION: Return true if everything was successful
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // ACTION: Create a getter for the `allowances` HashMap
            //   HINT: Take a look at the getters above if you forget the details
            // ACTION: Return the `allowance` value
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: Get the allowance for `(from, self.env().caller())` using `allowance_of_or_zero`
            // ACTION: `if` the `allowance` is less than the `value`, exit early and return `false`
            // ACTION: `insert` the new allowance into the map for `(from, self.env().caller())`
            // ACTION: Finally, call the `transfer_from_to` for `from` and `to`
            // ACTION: Return true if everything was successful
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
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
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 0);
        }

        #[test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x1; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x1; 32]), 100));
        }

        #[test]
        fn transfer_from_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 100);
            contract.approve(AccountId::from([0x0; 32]), 20);
            contract.transfer_from(AccountId::from([0x0; 32]), AccountId::from([0x1; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 10);
        }
    }
}