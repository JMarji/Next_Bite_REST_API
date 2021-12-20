//! src/configuration.rs

#![allow(dead_code, unused_imports)]

use color_eyre::Result;
use config;
use eyre::WrapErr;
use serde::Deserialize;
use std::convert::TryInto;
use tracing::{event, instrument, Level};
use tracing_subscriber::EnvFilter;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // That `config` knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}

/*
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub host: String,
}

impl Config {
    #[instrument]
    pub fn get_config() -> Result<Config> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let server_config = Config {
            host: String::from("127.0.0.1:7474"),
        };

        server_config
            .try_into()
            .context("loading configuration from environment")
    }
}

 */
