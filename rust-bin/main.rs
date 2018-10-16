#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, forbid(warnings))]

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
