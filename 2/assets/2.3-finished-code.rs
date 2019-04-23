#![cfg_attr(not(any(test, feature = "test-env")), no_std)]

use parity_codec::{
    Decode,
    Encode,
};
use ink_core::{
    env::{
        self,
        AccountId,
        Balance,
    },
    memory::format,
    storage,
};
use ink_lang::contract;

/// Events deposited by the ERC20 token contract.
#[derive(Encode, Decode)]
enum Event {
    Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    },
}

/// Deposits an ERC20 token event.
fn deposit_event(event: Event) {
    env::deposit_raw_event(&event.encode()[..])
}

contract! {
    struct Erc20 {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// The balance of each user.
        balances: storage::HashMap<AccountId, Balance>,
    }

    impl Deploy for Erc20 {
        fn deploy(&mut self, init_value: Balance) {
            self.total_supply.set(init_value);
            self.balances.insert(env.caller(), init_value);
            deposit_event(Event::Transfer { 
                from: None,
                to: Some(env.caller()),
                value: init_value
            });
        }
    }

    impl Erc20 {
        /// Returns the total number of tokens in existence.
        pub(external) fn total_supply(&self) -> Balance {
            let total_supply = *self.total_supply;
            env.println(&format!("Erc20::total_supply = {:?}", total_supply));
            total_supply
        }

        /// Returns the balance of the given address.
        pub(external) fn balance_of(&self, owner: AccountId) -> Balance {
            let balance = self.balance_of_or_zero(&owner);
            env.println(&format!("Erc20::balance_of(owner = {:?}) = {:?}", owner, balance));
            balance
        }

        /// Transfers token from the sender to the `to` address.
        pub(external) fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_impl(env.caller(), to, value)
        }
    }

    impl Erc20 {
        /// Returns the balance of the address or 0 if there is no balance.
        fn balance_of_or_zero(&self, of: &AccountId) -> Balance {
            let balance = self.balances.get(of).unwrap_or(&0);
            *balance
        }

        /// Transfers token from a specified address to another address.
        fn transfer_impl(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let balance_from = self.balance_of_or_zero(&from);
            let balance_to = self.balance_of_or_zero(&to);
            if balance_from < value {
                return false
            }
            self.balances.insert(from, balance_from - value);
            self.balances.insert(to, balance_to + value);
            deposit_event(Event::Transfer { 
                from: Some(from),
                to: Some(to),
                value: value
            });
            true
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn deployment_works() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        env::test::set_caller(alice);

        // Deploy the contract with some `init_value`
        let erc20 = Erc20::deploy_mock(1234);
        // Check that the `total_supply` is `init_value`
        assert_eq!(erc20.total_supply(), 1234);
        // Check that `balance_of` Alice is `init_value`
        assert_eq!(erc20.balance_of(alice), 1234);
    }

    #[test]
    fn transfer_works() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        env::test::set_caller(alice);
        // Deploy the contract with some `init_value`
        let mut erc20 = Erc20::deploy_mock(1234);
        // Alice does not have enough funds for this
        assert_eq!(erc20.transfer(bob, 4321), false);
        // Alice can do this though
        assert_eq!(erc20.transfer(bob, 234), true);
        // Check Alice and Bob have the expected balance
        assert_eq!(erc20.balance_of(alice), 1000);
        assert_eq!(erc20.balance_of(bob), 234);
    }
}
