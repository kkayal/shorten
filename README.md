# Objective:

A collection of convenience functions, macros and traits to shorten repetitive code.

# Status

Experimental

# Version

0.1.1

## Example 1:

Shorten the convertion to a String.

`s!("Hello")` is the same as `String::from("Hello"))`

## Example 2:

Concatenate two string(s) (slices) and return a **string slice**.

`ss!("Hello", ", world")` is the same as `"Hello, world";`

The same macro works also with an arbitrarz combination of String objects and string slices

```rust
// #[macro_use] extern crate shorten;
// let s1 = s!("Hello");
// let s2 = s!(", world");
// assert_eq!(ss!(s1, s2), "Hello, world");
```

## Example 3:
```rust
use shorten::*;
let data = fread("README.md").unwrap();
assert!(data.len() > 100);
```
