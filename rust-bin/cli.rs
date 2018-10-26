use clap_flags;
use failure::{Error, ResultExt};
use structopt::{self, StructOpt};

/// Command line parser.
#[derive(Debug, StructOpt)]
#[structopt(
  about = "{{DESCRIPTION}}",
  raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub struct Cli {
  #[structopt(flatten)]
  logger: clap_flags::Log,
  #[structopt(flatten)]
  verbosity: clap_flags::Verbosity,
}

impl Cli {
  /// Initialize a logger.
  #[inline]
  pub fn log(&self, name: &str) -> Result<(), Error> {
    self
      .logger
      .log(self.verbosity.log_level(), name)
      .context("Could not create logger")?;
    Ok(())
  }
}
