#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, forbid(warnings))]

//! ## Example
//! ```rust
//! ```

extern crate failure;

mod error;

pub use error::{Error, ErrorKind, Result};
