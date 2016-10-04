#![feature(plugin)]
#![plugin(clippy)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors;
use errors::*;
