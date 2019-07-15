#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;
use ink_core::storage;
use ink_core::env::DefaultSrmlTypes;

contract! {
    #![env = DefaultSrmlTypes]
    
    struct Incrementer {
        // ACTION: Create a `storage::Value` called `value` which holds a `u64`
    }

    impl Deploy for Incrementer {
        fn deploy(&mut self, init_value: u64) {
            // ACTION: `set` the initial value of `value` with `init_value`
        }
    }

    impl Incrementer {
        // Implementation of Contract Functions
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn incrementer_works() {
        let mut contract = Incrementer::deploy_mock(5);
    }
}
