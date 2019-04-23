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
    /// A transfer has been done.
    Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    },
    /// An approval for allowance was made.
    Approval {
        from: AccountId,
        to: AccountId,
        value: Balance,
    },
}

/// Deposits an ERC20 token event.
fn deposit_event(event: Event) {
    env::deposit_raw_event(&event.encode()[..])
}

contract! {
    /// The storage items for a typical ERC20 token implementation.
    struct Erc20 {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// All peeps done by all users.
        balances: storage::HashMap<AccountId, Balance>,
        /// Balances that are spendable by non-owners.
        ///
        /// # Note
        ///
        /// Mapping: (from, to) -> allowed
        allowances: storage::HashMap<(AccountId, AccountId), Balance>,
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

        /// Returns the amount of tokens that an owner allowed to a spender.
        pub(external) fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_or_zero(&owner, &spender)
        }

        /// Transfers token from the sender to the `to` address.
        pub(external) fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            env.println(&format!(
                "Erc20::transfer(to = {:?}, value = {:?})",
                to, value
            ));
            self.transfer_impl(env.caller(), to, value)
        }

        /// Approve the passed address to spend the specified amount of tokens
        /// on the behalf of the message's sender.
        pub(external) fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            env.println(&format!(
                "Erc20::approve(spender = {:?}, value = {:?})",
                spender, value
            ));
            let owner = env.caller();
            if owner == spender || value == 0 {
                return false
            }
            self.allowances.insert((owner, spender), value);
            Self::emit_approval(owner, spender, value);
            true
        }

        /// Transfer tokens from one address to another.
        pub(external) fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            env.println(&format!(
                "Erc20::transfer_from(from: {:?}, to = {:?}, value = {:?})",
                from, to, value
            ));
            let allowance = self.allowance_or_zero(&from, &env.caller());
            if allowance < value {
                return false
            }
            self.allowances.insert((from, env.caller()), allowance - value);
            self.transfer_impl(from, to, value)
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

        /// Returns the allowance or 0 of there is no allowance.
        fn allowance_or_zero(&self, from: &AccountId, to: &AccountId) -> Balance {
            let allowance = self.allowances.get(&(*from, *to)).unwrap_or(&0);
            env::println(&format!(
                "Erc20::allowance_or_zero(from = {:?}, to = {:?}) = {:?}",
                from, to, allowance
            ));
            *allowance
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
            Self::emit_transfer(from, to, value);
            true
        }
    }

    impl Erc20 {
        /// Emits a transfer event.
        fn emit_transfer<F, T>(
            from: F,
            to: T,
            value: Balance,
        )
        where
            F: Into<Option<AccountId>>,
            T: Into<Option<AccountId>>,
        {
            let (from, to) = (from.into(), to.into());
            assert!(from.is_some() || to.is_some());
            assert_ne!(from, to);
            assert!(value > 0);
            deposit_event(Event::Transfer { from, to, value });
        }
        
        /// Emits an approval event.
        fn emit_approval(
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) {
            assert_ne!(from, to);
            assert!(value > 0);
            deposit_event(Event::Approval { from, to, value });
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

    #[test]
    fn allowance_works() {
        let mut erc20 = Erc20::deploy_mock(1234);
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();
        let charlie = AccountId::try_from([0x2; 32]).unwrap();

        env::test::set_caller(alice);
        // Not allowed, since alice is the caller
        // and she has no approval from bob.
        assert_eq!(erc20.transfer_from(bob, alice, 1), false);
        assert_eq!(erc20.allowance(alice, bob), 0);
        assert_eq!(erc20.approve(bob, 20), true);
        assert_eq!(erc20.allowance(alice, bob), 20);

        // Charlie cannot send on behalf of Bob or Alice
        env::test::set_caller(charlie);
        assert_eq!(erc20.transfer_from(alice, bob, 10), false);
        // Bob cannot transfer more than he is allowed
        env::test::set_caller(bob);
        assert_eq!(erc20.transfer_from(alice, charlie, 25), false);
        // This should work though
        assert_eq!(erc20.transfer_from(alice, charlie, 10), true);
        // Allowance is updated
        assert_eq!(erc20.allowance(alice, bob), 10);
        // Balance transferred to the right person
        assert_eq!(erc20.balance_of(charlie), 10);

    }
}
