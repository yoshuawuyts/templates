#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, forbid(warnings))]

//! ## Example
//! ```rust
//! ```

#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;

mod cli;
mod error;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
