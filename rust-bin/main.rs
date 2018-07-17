#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate structopt;

mod cli;

use cli::Cli;
use structopt::StructOpt;

fn main() -> Result<(), Box<std::error::Error>> {
  let _args = Cli::from_args();
  Ok(())
}
