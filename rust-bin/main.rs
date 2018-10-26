#![forbid(unsafe_code, missing_debug_implementations)]
#![cfg_attr(test, forbid(warnings))]

extern crate human_panic;
extern crate structopt;
extern crate log;
extern crate {{PROJECTNAME}};
extern crate exitfailure;

use {{PROJECTNAME}}::Cli;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use human_panic::setup_panic;
use log::info;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;
  info!("program started");
  Ok(())
}
