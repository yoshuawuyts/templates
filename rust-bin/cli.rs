use clap_flags;
use structopt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
  #[structopt(flatten)]
  pub port: clap_flags::Port,
  #[structopt(flatten)]
  pub logger: clap_flags::Log,
  #[structopt(flatten)]
  pub verbosity: clap_flags::Verbosity,
}
