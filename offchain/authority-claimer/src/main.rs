// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use authority_claimer::config::Config;
use std::error::Error;
use tracing::info;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
 }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Getting the configuration.
    let config: Config = Config::new().map_err(Box::new)?;

    // Setting up the logging environment.
    log::configure(&config.authority_claimer_config.log_config);

    info!(?config, "Starting Authority Claimer");
    info!("Package Version : {}",built_info::PKG_VERSION);
    info!("Git Version: {:?}",built_info::GIT_VERSION);
    info!("Last git commit: {:?}",built_info::GIT_COMMIT_HASH);
    info!("Build time: {}",built_info::BUILT_TIME_UTC);


    authority_claimer::run(config).await
}
