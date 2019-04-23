#![no_std]

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

#[cfg(test)]
mod tests {
    use super::Incrementer;

    #[test]
    fn it_works() {
        // Test Your Contract
    }
}
