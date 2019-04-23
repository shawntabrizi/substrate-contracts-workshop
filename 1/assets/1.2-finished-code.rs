#![no_std]

use ink_lang::contract;
use ink_core::storage;

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
        // Implementation of Contract Functions
    }
}

#[cfg(test)]
mod tests {
    use super::Incrementer;

    #[test]
    fn it_works() {
        let mut contract = Incrementer::deploy_mock(5);
    }
}
