#![cfg_attr(not(any(test, feature = "test-env")), no_std)]

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

contract! {
    /// The storage items for a typical ERC20 token implementation.
    struct Erc20 {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// All peeps done by all users.
        balances: storage::HashMap<AccountId, Balance>,
    }

    impl Deploy for Erc20 {
        fn deploy(&mut self, init_value: Balance) {
            // We have to set total supply to `0` in order to initialize it.
            // Otherwise accesses to total supply will panic.
            env.println(&format!("Erc20::deploy(caller = {:?}, init_value = {:?})", env.caller(), init_value));
            self.total_supply.set(init_value);
            self.balances.insert(env.caller(), init_value);
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
            env.println(&format!(
                "Erc20::transfer(to = {:?}, value = {:?})",
                to, value
            ));
            self.transfer_impl(env.caller(), to, value)
        }
    }

    impl Erc20 {
        /// Returns the balance of the address or 0 if there is no balance.
        fn balance_of_or_zero(&self, of: &AccountId) -> Balance {
            let balance = self.balances.get(of).unwrap_or(&0);
            env::println(&format!(
                "Erc20::balance_of_or_zero(of = {:?}) = {:?}",
                of, balance
            ));
            *balance
        }

        /// Transfers token from a specified address to another address.
        fn transfer_impl(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            env::println(&format!(
                "Erc20::transfer_impl(from = {:?}, to = {:?}, value = {:?})",
                from, to, value
            ));
            let balance_from = self.balance_of_or_zero(&from);
            let balance_to = self.balance_of_or_zero(&to);
            if balance_from < value {
                return false
            }
            self.balances.insert(from, balance_from - value);
            self.balances.insert(to, balance_to + value);
            true
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn transfer_works() {
        let mut erc20 = Erc20::deploy_mock(1234);
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        env::test::set_caller(alice);
        assert_eq!(erc20.total_supply(), 1234);
        assert_eq!(erc20.balance_of(alice), 1234);
        // Alice does not have enough funds for this
        assert_eq!(erc20.transfer(bob, 4321), false);
        // Alice can do this though
        assert_eq!(erc20.transfer(bob, 234), true);
        assert_eq!(erc20.balance_of(alice), 1000);
        assert_eq!(erc20.balance_of(bob), 234);
    }
}
