# Objective:

A collection of convenience functions, macros and traits to shorten repetitive code.

# Status

Experimental

# Version

0.1.0

## Example 1:

Shorten the convertion to a String.

`s!("Hello")` is the same as `String::from("Hello"))`

## Example 2:

Concatenate two string(s) (slices) and return a **string slice**.

`ss!("Hello", ", world")` is the same as `"Hello, world"`

The same macro works also with an arbitrary combination of String objects and string slices

```
#[macro_use] extern crate shorten;
let s1 = s!("Hello");
let s2 = s!(", world");
assert_eq!(ss!(s1, s2), "Hello, world");
```
