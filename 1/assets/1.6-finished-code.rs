#![cfg_attr(not(any(test, feature = "test-env")), no_std)]

use ink_lang::contract;
use ink_core::storage;
use ink_core::memory::format;
use ink_core::env::{self, println, AccountId};

contract! {
    struct Incrementer {
        value: storage::Value<u64>,
        my_value: storage::HashMap<AccountId, u64>,
    }

    impl Deploy for Incrementer {
        fn deploy(&mut self, init_value: u64) {
            self.value.set(init_value)
        }
    }

    impl Incrementer {
        pub(external) fn get(&self) -> u64 {
            println(&format!("Incrementer::get = {:?}", *self.value));
            *self.value
        }

        pub(external) fn inc(&mut self, by: u64) {
            println(&format!("Incrementer::inc by = {:?}", by));
            let my_value = self.value_of_or_default(&env.caller());
            let new_value = my_value + by;
            self.my_value.insert(env.caller(), new_value);
        }

        pub(external) fn get_mine(&self) -> u64 {
            let my_value = self.value_of_or_default(&env.caller());
            println(&format!("Incrementer::get_mine = {:?}", my_value));
            my_value
        }
    }

    impl Incrementer {
        fn value_of_or_default(&self, of: &AccountId) -> u64 {
            let my_value = self.my_value.get(of).unwrap_or(&self.value);
            println(&format!(
                "Incrementer::value_of_or_default(of = {:?}) = {:?}",
                of, my_value
            ));
            *my_value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn it_works() {
        let mut contract = Incrementer::deploy_mock(5);
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        env::test::set_caller(alice);
        assert_eq!(contract.get(), 5);
        assert_eq!(contract.get_mine(), 5);
        contract.inc(42);
        assert_eq!(contract.get_mine(), 47);
        contract.inc(0);
        assert_eq!(contract.get_mine(), 47);
        
        env::test::set_caller(bob);
        assert_eq!(contract.get_mine(), 5);
    }
}
