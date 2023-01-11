//! CLI subcommands

use clap::{ArgMatches, Command};

use sherbert::config::Config;
use sherbert::errors::CliResult;

pub mod metadata;

/// Build all builtin commands
pub fn builtin() -> Vec<Command> {
    vec![metadata::cli()]
}

/// Find builtin executor to exec
pub fn builtin_exec(cmd: &str) -> Option<fn(&mut Config, &ArgMatches) -> CliResult> {
    let function = match cmd {
        "metadata" => metadata::exec,
        _ => return None,
    };
    Some(function)
}
