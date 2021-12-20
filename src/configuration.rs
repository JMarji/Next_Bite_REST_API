#![allow(dead_code, unused_imports)]

use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use std::convert::TryInto;
use tracing::{event, instrument, Level};
use tracing_subscriber::EnvFilter;

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
