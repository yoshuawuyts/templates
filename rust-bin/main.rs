#![forbid(unsafe_code, bad_style, future_incompatible)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![cfg_attr(test, deny(warnings))]

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
