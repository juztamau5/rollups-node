// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use clap::Parser;

use inspect_server::{config::CLIConfig, InspectServerConfig};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: InspectServerConfig = CLIConfig::parse().into();

    logs::configure(&config.logs_config);

    info!(?config, "starting inspect server");

    inspect_server::run(config).await.map_err(|e| e.into())
}
