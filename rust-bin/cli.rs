use clap_flags;
use structopt;

/// Command line parser.
#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
  #[structopt(flatten)]
  pub logger: clap_flags::Log,
  #[structopt(flatten)]
  pub verbosity: clap_flags::Verbosity,
}
