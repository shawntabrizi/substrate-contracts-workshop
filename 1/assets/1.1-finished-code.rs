#![cfg_attr(not(any(test, feature = "std")), no_std)]

// Import the `contract!` macro
use ink_lang::contract;
use ink_core::env::DefaultSrmlTypes;

contract! {
    #![env = DefaultSrmlTypes]
    
    struct Incrementer {
        // Storage Declaration
    }

    impl Deploy for Incrementer {
        fn deploy(&mut self) {
            // Contract Constructor
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
        // Test Your Contract
    }
}
