#![forbid(clippy::missing_docs_in_private_items)]
#![warn(rust_2018_idioms)]

//! # Sherbert
//!
//! Opinionated project tooling.

use figment::providers::Serialized;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use sherbert::config::Config;

mod cli;
mod commands;

/// Entrypoint.
fn main() {
    let mut config_toml_path = dirs::config_dir().expect("Failed to get user config directory");
    config_toml_path.push("letsbuilda");
    config_toml_path.push("sherbert.toml");

    let mut config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(config_toml_path).nested())
        .merge(Env::prefixed("SHERBERT_"))
        .extract()
        .unwrap();

    let result = cli::main(&mut config);
    match result {
        Ok(()) => {}
        Err(err) => println!("{err:?}"),
    }
}
