Transferring Tokens
===

So at this point, we have a single user which owns all the tokens for the contract. However, it's not _really_ a token unless you can transfer them to other people... Let's do that!

## The Transfer Function

The `transfer` function does exactly what you might expect: it allows the user calling the contract to transfer some funds they own to another user.

You will notice in our template code there is a public function `transfer` and an internal function `transfer_impl`. We have done this because in the future, we will be reusing the logic for a token transfer when we enable third party allowances and spending on-behalf-of.

The `transfer_impl` function will be built without any _authorization_ checks. Because it is an internal function, we fully control when it gets called. However, it will have all logical checks around managing the balances between accounts, and really we just need to check for one thing: make sure that the `from` account has enough funds to send to the `to` account.

The `transfer` function will simply call into the `t


- Talk about seperating the `transfer_impl` because it is used in `transfer_from`

- Talk about the simple math used to calculate balances
- Talk about how we confirm that math is safe
    - Through checks
    - Through contract logic

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->