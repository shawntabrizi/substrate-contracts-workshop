#![no_std]

use ink_lang::contract;
use ink_core::storage;
use ink_core::env::println;
use ink_core::memory::format;
use ink_core::env::AccountId;

contract! {
    struct Incrementer {
        value: storage::Value<u32>,
        my_value: storage::HashMap<AccountId, u32>,
    }

    impl Deploy for Incrementer {
        fn deploy(&mut self, init_value: u32) {
            self.value.set(init_value)
        }
    }

    impl Incrementer {
        pub(external) fn get(&self) -> u32 {
            println(&format!("Incrementer::get = {:?}", *self.value));
            *self.value
        }

        pub(external) fn inc(&mut self, by: u32) {
            println(&format!("Incrementer::inc by = {:?}", by));
            self.value += by;
        }

        pub(external) fn get_mine(&self) -> u32 {
            let my_value = self.value_of_or_default(&env.caller());
            println(&format!("Incrementer::get_mine = {:?}", my_value));
            my_value
        }
    }

    impl Incrementer {
        fn value_of_or_default(&self, of: &AccountId) -> u32 {
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
    use super::Incrementer;

    #[test]
    fn it_works() {
        let contract = Incrementer::deploy_mock(5);
        assert_eq!(contract.get(), 5);
        assert_eq!(contract.get_mine(), 5);
    }
}
