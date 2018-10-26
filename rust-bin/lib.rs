#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, forbid(warnings))]

//! ## Example
//! ```rust
//! ```

extern crate clap_flags;
extern crate failure;
extern crate structopt;

mod cli;
mod error;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
