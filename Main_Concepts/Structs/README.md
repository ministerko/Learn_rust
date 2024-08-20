# Structs
To define a `struct`, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call *fields*. For example, 

```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```
To get a specific value from a `struct`, we use `dot notation`. For example, to access this user’s email address, we use `user1.email`. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field



[see codes](./struct_intro.rs)

Note that the entire instance must be `mutable`; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
