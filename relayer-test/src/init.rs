use eyre::Report as Error;
use std::env;
use tracing_subscriber::{
    self as ts,
    filter::EnvFilter,
    layer::{Layer, SubscriberExt},
    util::SubscriberInitExt,
};

use crate::config::TestConfig;

pub fn init_test() -> Result<TestConfig, Error> {
    color_eyre::install()?;
    install_logger();

    let chain_command_path = env::var("CHAIN_COMMAND_PATH").unwrap_or_else(|_| "gaiad".to_string());

    let chain_store_dir = env::var("CHAIN_STORE_DIR").unwrap_or_else(|_| "data".to_string());

    Ok(TestConfig {
        chain_command_path,
        chain_store_dir,
    })
}

fn install_logger() {
    // Use log level INFO by default if RUST_LOG is not set.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let module_filter_fn = ts::filter::filter_fn(|metadata| match metadata.module_path() {
        Some(path) => path.starts_with("ibc"),
        None => false,
    });

    let module_filter = ts::fmt::layer().with_filter(module_filter_fn);

    ts::registry().with(env_filter).with(module_filter).init();
}
