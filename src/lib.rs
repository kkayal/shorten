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
//! 0.1.4
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
//! // use shorten::*;
//! // let s1 = s!("Hello");
//! // let s2 = s!(", world");
//! // assert_eq!(ss!(s1, s2), "Hello, world");
//! ```
//! 
//! ## Example 3:
//! // #[macro_use] extern crate shorten;
//! ```rust
//! use shorten::*;
//! let v = fread(".gitignore").unwrap();
//!
//! // Convert to a String
//!
//! // Option 1: Safe. Consumes the vector and complains about invalid UTF-8
//! let s = String::from_utf8(v).expect("Found invalid UTF-8");
//!
//! // Option 2: Lossy. It turns invalid UTF-8 bytes into � and so no error handling is required
//! let v = fread(".gitignore").unwrap();
//! let s = String::from_utf8_lossy(&v[..]).into_owned();
//!
//! // Option 3: Convert to a u8 array (not Vec) and then to a string slice. 
//! // The conversion is in-place, and does not require an allocation. You can create a String from the slice if necessary.
//! let v = fread(".gitignore").unwrap();
//! let s = std::str::from_utf8(&v[..]).unwrap();
//! ```

// Shorten String::from("hello") to s!("hello")
#[macro_export]
macro_rules! s { ( $x:expr ) => { String::from($x); }; }

// Concatenate two string(s) (slices) and return a string slice
#[macro_export]
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
		let v = fread(".gitignore").unwrap();
		println!("v = {:?}", v);
		assert_eq!(v, [47, 116, 97, 114, 103, 101, 116, 47, 13, 10, 67, 97, 114, 103, 111, 46, 108, 111, 99, 107, 13, 10]);

		// Convert to a String
		// Option 1: Safe
		// Thanks to https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
		let s = String::from_utf8(v).expect("Found invalid UTF-8");
		assert_eq!(s, "/target/\r\nCargo.lock\r\n");

		// Option 2: Lossy
		// It turns invalid UTF-8 bytes into � and so no error handling is required
		let v = fread(".gitignore").unwrap(); // Read it again, since "v" was consumed above
		let s = String::from_utf8_lossy(&v[..]); // Does NOT consume v ???
		assert_eq!(s, "/target/\r\nCargo.lock\r\n");

		// Option 3: Convert to a u8 array (not Vec) and then to a string slice. 
		// The conversion is in-place, and does not require an allocation. You can create a String from the slice if necessary.
		let s = std::str::from_utf8(&v[..]).unwrap(); // Does NOT consume v ???
		assert_eq!(s, "/target/\r\nCargo.lock\r\n");
	}
}
