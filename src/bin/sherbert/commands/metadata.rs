//! Metadata.

use std::env;

use clap::{ArgMatches, Command};

use sherbert::config::Config;
use sherbert::errors::CliResult;
use sherbert::paths::find_file;

/// Build CLI for `metadata` subcommand.
pub fn cli() -> Command {
    Command::new("metadata").about("Output project metadata")
}

/// Exec `metadata` action.
pub fn exec(config: &mut Config, _args: &ArgMatches) -> CliResult {
    let path = env::current_dir()?;
    let file = find_file(path, "Cargo.toml");
    if config.log_level != -1 {
        println!("{file:?}");
    }
    Ok(())
}
