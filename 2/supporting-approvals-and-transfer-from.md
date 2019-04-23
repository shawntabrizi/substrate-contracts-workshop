Supporting Approvals and Transfer From
===

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