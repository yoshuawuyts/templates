#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

#[macro_use]
extern crate human_panic;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate {{PROJECTNAME}};
extern crate exitfailure;

use {{PROJECTNAME}}::Cli;
use structopt::StructOpt;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;
  info!("program started");
  Ok(())
}
