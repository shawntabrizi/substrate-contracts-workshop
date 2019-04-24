Incrementing the Value
===

### Mutable and Immutable Functions

You may have noticed that the function templates included `self` as the first parameter of the contract functions. It is through `self` that you gain access to all your contract functions and storage items.

If you are simply reading from the contract storage, you only need to pass `&self`. But if you want to modify storage items, you will need to explicitly mark it as mutable, `&mut self`.

```rust
impl MyContract {
    // Public functions go here
    pub(external) fn my_getter(&self) -> u32{
        *self.my_number
    } 

    pub(external) fn my_setter(&mut self, some_value: u32) {
        self.my_number = some_value;
    }
}
```


- Talk about the importance of initializing a value
- Talk about modifying a value
- Using `get()` and `set()` versus `+=`

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.4-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->