Storing a Value
===

The entire ink! smart contract is built off of a `struct` which defines your contract storage.

Here is how you would store some simple values in storage:

```rust
contract! {
    struct MyContract {
        // Store a bool
        my_bool: storage::Value<bool>,
        // Store some number
        my_number: storage::Value<u32>,
    }
    ...
}
```

## Supported Types

Contract storage like `storage::Value<T>` is allowed to be generic over types that are encodable and decodable with [Parity Codec](https://github.com/paritytech/parity-codec) which includes the most common types such as `bool`, `u{16,32,64,128}`, `i{8,16,32,64,128}`, `String`, tuples, and arrays.  Note that `u8` is [not currently supported](https://github.com/paritytech/parity-codec/issues/47) in `parity_codec`.

ink! also supports Substrate specific types like `AccountId`, `Balance`, and `Hash`. To use some of these non-primitive types, we have to import them from `ink_core::env::DefaultSrmlTypes`. We also have to include `#![env = DefaultSrmlTypes]` to the beginning of our `contract!` macro.

Here is an example of how you would store an `AccountId` and `Balance`:

```rust
// We are importing the default SRML types
use ink_core::env::DefaultSrmlTypes;

contract! {
    #![env = DefaultSrmlTypes]

    // Our struct will use those default SRML types
    struct MyContract {
        // Store some AccountId
        my_account: storage::Value<AccountId>,
        // Store some Balance
        my_balance: storage::Value<Balance>,
    }
    ...
}
```

You can find all the supported Substrate types in the [`core/env/traits.rs` file](https://github.com/paritytech/ink/blob/master/core/src/env/traits.rs).

## Initializing Storage

> **IMPORTANT:** This section is important. Read it twice. Then have tea and read it again.

Before you can interact with any storage items in an ink! contract, **you must make sure they are initialized!** If you do not initialize a storage item and try to access it, your contract call will not succeed and any state changes caused by the transaction will be reverted. (Gas fees will still be charged though!)

For storage values like the ones above, we can set an initial value with:

```rust
self.my_bool.set(false);
self.my_number.set(42);
self.my_account.set(AccountId::from([0x0; 32]));
self.my_balance.set(1337);
```

This can be done anywhere in our contract logic, but most commonly this happens in the `Deploy` section.

## Contract Deployment

Every ink! smart contract must implement the `Deploy` trait which consists of a single function, `deploy`, which is run once when a contract is created.

```rust
contract! {
    #![env = DefaultSrmlTypes]

    struct MyContract {
        ...
    }

    impl Deploy for MyContract {
        fn deploy(&mut self) {
            // Deployment logic that runs once upon contract creation
        }
    }
}
```

> **Note:** If you are familiar with Solidity, this is similar to the `constructor` function, however in ink!, `deploy` is not optional.

### Deployment Variables

You can deploy a contract with some configurable parameters so that users can customize the contract they are instantiating. You can deploy a contract using one or more parameters like so:

```rust
contract! {
    #![env = DefaultSrmlTypes]

    struct MyContract {
        // Store a number
        my_number: storage::Value<u32>,
        // Store some Balance
        my_balance: storage::Value<Balance>,
    }

    impl Deploy for MyContract {
        /// Allows the user to initialize `my_number` with an input value
        fn deploy(&mut self, init_value: u32, init_balance: Balance) {
            self.my_number.set(init_value);
            self.my_balance.set(init_balance);
        }
    }
    ...
}
```

> **Note:** Parameter types of `deploy` and other contract messages are very restricted. We currently only allow users to pass primitives such as `bool`, `u{8,16,32,64,128}`, `i{8,16,32,64,128}` as well as SRML primitives such as `AccountId` and `Balance`.

## Your Turn!

Follow the `ACTION`s in the template.

TODO: Add more content.

Remember to run `cargo test --features test-env` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
