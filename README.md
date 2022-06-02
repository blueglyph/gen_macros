# gen_macros

A set of general-purpose Rust macros.

## s

Makes the string argument a String, by using `String::from()` (see `String` for details).

### Example
```rust
let text: String = s!("Hello");
```

## hashmap

Generates the code to initialize a `HashMap`.

### Example
```rust
let days = hashmap!(0 => "Monday", 1 => "Tuesday", 2 => "Wednesday");
```
