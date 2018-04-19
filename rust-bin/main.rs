#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate quicli;

use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Args {}

main!(|_args: Args| {});
