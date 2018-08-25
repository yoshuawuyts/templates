#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate failure;

mod error;

pub use error::{Error, ErrorKind, Result};
