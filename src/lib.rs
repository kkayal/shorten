//! # Objective:
//! 
//! A collection of convenience functions, macros and traits to shorten repetitive code.
//! 
//! # Status
//! 
//! Experimental
//! 
//! # Version
//! 
//! 0.1.1
//! 
//! ## Example 1:
//! 
//! Shorten the convertion to a String.
//! 
//! `s!("Hello")` is the same as `String::from("Hello"))`
//! 
//! ## Example 2:
//! 
//! Concatenate two string(s) (slices) and return a **string slice**.
//! 
//! `ss!("Hello", ", world")` is the same as `"Hello, world";`
//! 
//! The same macro works also with an arbitrarz combination of String objects and string slices
//! 
//! ```rust
//! // #[macro_use] extern crate shorten;
//! // let s1 = s!("Hello");
//! // let s2 = s!(", world");
//! // assert_eq!(ss!(s1, s2), "Hello, world");
//! ```
//! 
//! ## Example 3:
//! ```rust
//! use shorten::*;
//! let data = fread("README.md").unwrap();
//! assert!(data.len() > 100);
//! ```

// Shorten String::from("hello") to s!("hello")
macro_rules! s { ( $x:expr ) => { String::from($x); }; }

// Concatenate two string(s) (slices) and return a string slice
macro_rules! ss {	($x:expr, $y:expr) => ( &*format!("{}{}", $x, $y); ) }

use std::fs::File;
use std::path::Path;
use std::io;

#[allow(unused_imports)]
use std::io::{Read, Write};

/// Read a file into a Vec[u8]
pub fn fread<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
  let mut file = File::open(path)?;
  let meta = file.metadata()?;
  let size = meta.len() as usize;
  let mut data = Vec::with_capacity(size);
  data.resize(size, 0);
  file.read_exact(&mut data)?;
  Ok(data)
}

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

	#[test]
	fn fread_test() {
		let data = fread("README.md").unwrap();
		assert!(data.len() > 100);
	}
}
