# Objective:

A collection of convenience functions, macros and traits to shorten repetitive code.

# Status

Experimental

# Version

0.1.3

## Example 1:

Shorten the convertion to a String.

`s!("Hello")` is the same as `String::from("Hello"))`

## Example 2:

Concatenate two string(s) (slices) and return a **string slice**.

`ss!("Hello", ", world")` is the same as `"Hello, world";`

The same macro works also with an arbitrarz combination of String objects and string slices

```rust
// #[macro_use] extern crate shorten;
// use shorten::*;
// let s1 = s!("Hello");
// let s2 = s!(", world");
// assert_eq!(ss!(s1, s2), "Hello, world");
```

## Example 3:
// #[macro_use] extern crate shorten;
```rust
use shorten::*;
let v = fread(".gitignore").unwrap();
// Convert to a String
// Option 1: Safe. Consumes the vector and complains about invalid UTF-8
let s = String::from_utf8(v).expect("Found invalid UTF-8");
// Option 2: Lossy. It turns invalid UTF-8 bytes into � and so no error handling is required
let s = String::from_utf8_lossy(&v[..]).into_owned(); // Does NOT consume v ???
// Option 3: Convert to a u8 array (not Vec) and then to a string slice. 
// The conversion is in-place, and does not require an allocation. You can create a String from the slice if necessary.
let s = std::str::from_utf8(&v[..]).unwrap(); // Does NOT consume v ???
```
