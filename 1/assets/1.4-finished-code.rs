#![no_std]

use ink_lang::contract;
use ink_core::storage;
use ink_core::env::println;
use ink_core::memory::format;

contract! {
    struct Incrementer {
        value: storage::Value<u64>,
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
            self.value += by;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Incrementer;

    #[test]
    fn it_works() {
        let mut contract = Incrementer::deploy_mock(5);
        assert_eq!(contract.get(), 5);
        contract.inc(42);
        assert_eq!(contract.get(), 47);
        contract.inc(0);
        assert_eq!(contract.get(), 47);
    }
}
