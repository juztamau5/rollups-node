// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use log;
use tracing::info;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
 }

// NOTE: doesn't support History upgradability.
// NOTE: doesn't support changing epoch_duration in the middle of things.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = dispatcher::config::Config::initialize()?;

    log::configure(&config.dispatcher_config.log_config);

    info!(?config, "Starting Dispatcher");

    
    info!("Package Version : {}",built_info::PKG_VERSION);
    info!("Git Version: {:?}",built_info::GIT_VERSION);
    info!("Last git commit: {:?}",built_info::GIT_COMMIT_HASH);
    info!("Build time: {}",built_info::BUILT_TIME_UTC);

    dispatcher::run(config).await.map_err(|e| e.into())
}
