use std::str::FromStr;
use structopt::{clap::Shell, StructOpt};

include!("src/lib.rs");

fn main() -> Result<(), Box<std::error::Error>> {
  let outdir = ::std::env::var_os("OUT_DIR").expect("OUT_DIR not found.");
  let mut app = cli::Opts::clap();
  for shell in Shell::variants().into_iter() {
    let shell = Shell::from_str(*shell)?;
    app.gen_completions(env!("CARGO_PKG_NAME"), shell, &outdir);
  }
  Ok(())
}
