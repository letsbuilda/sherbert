//! Struct to hold internal config

use clap::ArgMatches;
use serde::{Deserialize, Serialize};

/// Configuration information for sherbert. This is not specific to a build, it is information
/// relating to sherbert itself.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// The level of messages to log.
    pub log_level: i8,
}

impl Config {
    /// Creates a new config instance.
    pub fn new(log_level: i8) -> Config {
        Config { log_level }
    }

    /// Update the config from the CLI matches.
    pub fn update_from_matches(&mut self, matches: &ArgMatches) {
        self.log_level = if *matches.get_one::<bool>("quiet").unwrap() {
            -1
        } else {
            matches.get_count("verbose") as i8
        };
    }

    /// Get the configured log level.
    pub fn level_filter(&self) -> tracing_subscriber::filter::LevelFilter {
        match self.log_level {
            -1 => tracing_subscriber::filter::LevelFilter::OFF,
            0 => tracing_subscriber::filter::LevelFilter::ERROR,
            1 => tracing_subscriber::filter::LevelFilter::WARN,
            2 => tracing_subscriber::filter::LevelFilter::INFO,
            3 => tracing_subscriber::filter::LevelFilter::DEBUG,
            _ => tracing_subscriber::filter::LevelFilter::TRACE,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::new(0)
    }
}
