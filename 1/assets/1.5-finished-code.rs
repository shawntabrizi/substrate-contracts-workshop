#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;
use ink_core::storage;
use ink_core::env::DefaultSrmlTypes;
use ink_core::memory::format;

contract! {
    #![env = DefaultSrmlTypes]
    
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
            env.println(&format!("Incrementer::get = {:?}", *self.value));
            *self.value
        }

        pub(external) fn inc(&mut self, by: u64) {
            self.value += by;
        }

        pub(external) fn get_mine(&self) -> u64 {
            let my_value = self.my_value_or_zero(&env.caller());
            env.println(&format!("Incrementer::get_mine = {:?}", my_value));
            my_value
        }
    }

    impl Incrementer {
        fn my_value_or_zero(&self, of: &AccountId) -> u64 {
            let my_value = self.my_value.get(of).unwrap_or(&0);
            *my_value
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn incrementer_works() {
        let contract = Incrementer::deploy_mock(5);
        assert_eq!(contract.get(), 5);
        assert_eq!(contract.get_mine(), 0);
    }
}
