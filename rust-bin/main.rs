#![feature(external_doc)]
#![doc(include = "../README.md")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate quicli;

use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Args {}

main!(|_args: Args| {});
