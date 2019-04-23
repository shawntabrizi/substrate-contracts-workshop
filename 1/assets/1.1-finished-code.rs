#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;

contract! {
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
    use super::Incrementer;

    #[test]
    fn incrementer_works() {
        // Test Your Contract
    }
}
