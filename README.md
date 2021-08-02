# Objective:

A collection of convenience functions, macros and traits to shorten repetitive code.

# Status

passively-maintained

# Version

0.2

## Example 1:

Shorten the conversion to a String.

`s!("Hello")` is the same as `String::from("Hello"))`

## Example 2:

Concatenate two string(s) (slices) and return a **string slice**.

`ss!("Hello", ", world")` is the same as `"Hello, world";`

The same macro works also with an arbitrary combination of String objects and string slices

```rust
#[macro_use] extern crate shorten;
use shorten::*;
let s1 = s!("Hello");
let s2 = s!(", world");
assert_eq!(ss!(s1, s2), "Hello, world");
```

## Example 3:
// #[macro_use] extern crate shorten;
```rust
#[macro_use] extern crate shorten;
use shorten::*;
let s1 = s!("Hello");
let s2 = s!(", world");
assert_eq!(ss!(s1, s2), "Hello, world");
```
