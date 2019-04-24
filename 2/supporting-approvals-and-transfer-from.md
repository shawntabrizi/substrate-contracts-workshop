Supporting Approvals and Transfer From
===

We are almost there! Our token contract can now transfer funds from user to user and tell the outside world what is going on when this happens. All that is left to do is introduce the `approve` and `transfer_from` functions.

## Third Parity Transfers

This section is all about adding the ability for other accounts to safely spend some amount of your tokens.

The immediate question should be: "Why the heck would I want that?"

Well, one such scenario is to support Decentralized Exchanges. Basically, other smart contracts can allow you to exchange tokens with other users, usually one type of token for another. However, these "bids" do not always exectute right away. Maybe you want to get a really good deal for token trade, and will hold out until that trade is met.

Well, rather than giving your tokens directly to the contract (an escrow), you can simply "approve" them to spend some of your tokens on your behalf! This means that during the time while you are waiting for a trade to execute, you can still control and spend your funds if needed. Better yet, you can approve multiple different contracts or users to access your funds, so if one contract offers the good trade, you do not need to pull out funds from the other and move them, a sometimes costly and time consuming process.

So hopefully you can see why a feature like this would be useful, but how can we do it safely?

We use a two step process: Approve and Transfer From.

### Approve

Approving another account to spend your funds is the first step in the third party transfer process. A token owner can specify another account and any arbitrary number of tokens that account can spend on the owner's behalf. The owner need not have all the funds approved to spend; in the situation where there is not enough funds, the approved account can spend up to what the owner's balance is.

When an account calls `approve` multiple times, the approved value simply overwrites any existing value that was approved in the past. By default, the approved value between any two accounts is `0`, and a user can always call approve for `0` to revoke access to their funds from another account.

To store approvals in our contract, we need to use a slightly fancy HashMap.

Since each account can have a different approval amount for any other account, we need to use a tuple as our key which simply points to a balance value. Here is an example of what that would look like:

```rust
struct Erc20 {
    /// Balances that are spendable by non-owners: (owner, spender) -> allowed
    allowances: storage::HashMap<(AccountId, AccountId), Balance>,
}
```

Here we have defined the tuple to represent `(owner, spender)` such that we can look up how much a "spender" can spend from an "owner's" balance using they `AccountId`s in this tuple. Remember that we will need to again create an `allowance_or_zero` function to help us get the allowance of an account when it is not initialized, and a getter function called `allowance` to look up the current value for any pair of accounts.

```rust
/// Approve the passed AccountId to spend the specified amount of tokens
/// on the behalf of the message's sender.
pub(external) fn approve(&mut self, spender: AccountId, value: Balance) -> bool {...}
```

When you call the `approve` function, you simply insert the `value` specified into storage. The `owner` is always the `env.caller()`, ensuring that the function call is always authorized.




- Talk about what `approve` and `tranfer_from` do
- Talk about the new storage item
    - Talk about using a tuple
    - Creating a new `allowance_or_zero`
    - Creating a new getter
- Talk about the trickiness around checking approval with `env.caller()` but then making transfers with `to` and `from`
- Everything else should be mostly the same


<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.4-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->