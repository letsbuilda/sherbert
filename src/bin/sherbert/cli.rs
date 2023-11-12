//! CLI

use clap::{command, Arg, ArgAction, ArgMatches, Command};

use sherbert::config::Config;
use sherbert::errors::CliResult;

use crate::commands;

/// Build the main CLI
pub(crate) fn build_cli() -> Command {
    command!()
        .infer_long_args(true)
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .conflicts_with("quiet")
                .action(ArgAction::Count)
                .help("Sets the level of verbosity")
                .global(true),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .conflicts_with("verbose")
                .action(ArgAction::SetTrue)
                .help("Suppresses all output")
                .global(true),
        )
        .subcommands(commands::builtin())
}

/// Init logging subscriber.
fn init_logging(log_level: tracing_subscriber::filter::LevelFilter) {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .with_max_level(log_level)
        .init();
}

/// Subcommand entrypoint.
fn execute_subcommand(config: &mut Config, cmd: &str, subcommand_args: &ArgMatches) -> CliResult {
    if let Some(exec) = commands::builtin_exec(cmd) {
        exec(config, subcommand_args)
    } else {
        Ok(())
    }

    // let mut ext_args: Vec<&OsStr> = vec![OsStr::new(cmd)];
    // ext_args.extend(
    //     subcommand_args
    //         .get_many::<OsString>("")
    //         .unwrap_or_default()
    //         .map(OsString::as_os_str),
    // );
    // super::execute_external_subcommand(config, cmd, &ext_args)
}

/// CLI entrypoint.
pub fn main(config: &mut Config) -> CliResult {
    let matches = build_cli().get_matches();

    config.update_from_matches(&matches);

    init_logging(config.level_filter());

    let (cmd, subcommand_args) = match matches.subcommand() {
        Some((cmd, args)) => (cmd, args),
        _ => {
            return Ok(());
        }
    };

    // tracing::error!("{:#?}", "error");
    // tracing::warn!("{:#?}", "warn");
    // tracing::info!("{:#?}", "info");
    // tracing::debug!("{:#?}", "debug");
    // tracing::trace!("{:#?}", "trace");

    execute_subcommand(config, cmd, subcommand_args)
}

#[test]
fn verify_cli() {
    build_cli().debug_assert();
}
