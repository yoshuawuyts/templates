#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate structopt;
extern crate {{PROJECTNAME}};

use {{PROJECTNAME}}::cli::Cli;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _args = Cli::from_args();
  Ok(())
}
