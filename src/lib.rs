//! # Objective:
//! 
//! A collection of convenience functions, macros and traits to shorten repetitive code.
//! 
//! # Status
//! 
//! passively-maintained
//! 
//! # Version
//! 
//! 0.2
//! 
//! ## Example 1:
//! 
//! Shorten the conversion to a String.
//! 
//! `s!("Hello")` is the same as `String::from("Hello"))`
//! 
//! ## Example 2:
//! 
//! Concatenate two string(s) (slices) and return a **string slice**.
//! 
//! `ss!("Hello", ", world")` is the same as `"Hello, world";`
//! 
//! The same macro works also with an arbitrary combination of String objects and string slices
//! 
//! ```rust
//! #[macro_use] extern crate shorten;
//! use shorten::*;
//! let s1 = s!("Hello");
//! let s2 = s!(", world");
//! assert_eq!(ss!(s1, s2), "Hello, world");
//! ```
//! 

// Shorten `String::from("hello")` to `s!("hello")`
#[macro_export]
macro_rules! s { ( $x:expr ) => { String::from($x); }; }

// Concatenate two string(s) (slices) and return a string slice
#[macro_export]
macro_rules! ss {	($x:expr, $y:expr) => ( &*format!("{}{}", $x, $y); ) }

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn s() {
		assert_eq!(s!("Hello"), String::from("Hello"));
	}

	#[test]
	fn ss() {
		assert_eq!(ss!("Hello", ", world"), "Hello, world");

		let s1 = s!("Hello");
		let s2 = s!(", world");
		assert_eq!(ss!(s1, s2), "Hello, world");
	}
}
