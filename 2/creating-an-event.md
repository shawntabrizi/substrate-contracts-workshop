Creating an Event
===

- Talk about the `Event` enum
    - Talk about filling it with events and syntax
- Talk about `deposit_event` and `deposit_raw_event`
- Talk about `Option<AccountId>`, `None`, `Some()`
- Talk about where we place the transfer event:
    - `transfer_impl`
    - `deploy`
- **NEW** Event syntax for event definition and emitting
    - Example: [**click**](https://github.com/paritytech/ink/blob/master/examples/lang/events/src/lib.rs)

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.3-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/2.2-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->