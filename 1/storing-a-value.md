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

Contract storage like `storage::Value<T>` is allowed to be generic over types that are encodable and decodable with [Parity Codec](https://github.com/paritytech/parity-codec) which includes the most common types such as `bool`, `u{8,16,32,64,128}`, `i{8,16,32,64,128}`, `String`, tuples, and arrays.

ink! also supports Substrate specific types like `AccountId`, `Balance`, and `Hash`. To use some of these non-primitive types, we have to import them from `ink_core::env`. Here is an example of how you would store an `AccountId` and `Balance`:

```rust
// Note that you will need to import `AccountId` and/or `Balance` to use them
use ink_core::env::{AccountId, Balance};

contract! {
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

> **IMPORTANT:** This section is important. Read it twice. Then, have tea and read it again.

Before you can interact with any storage items in an ink! contract, **you must make sure they are initialized!** If you do not initialize a storage item and try to access it, your contract call will not succeed and any state changes caused by the transaction will be reverted. (Gas fees will still be charged though!)

For storage values like the ones above, we can set an initial value with:

```rust
self.my_bool.set(false);
self.my_number.set(42);
self.my_account.set(AccountId::try_from([0x0; 32]).unwrap())
self.my_balance.set(1337);
```

- Talk about creating a storage value
- Talk about needing to initialize it
- Available types

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->