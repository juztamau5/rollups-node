// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)
mod config;
use config::Config;
use types::foldables::authority::rollups::RollupsState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::initialize_from_args()?;

    logs::configure(&config.logs_config);

    tracing::info!(?config, "starting state server");

    state_server::run_server::<RollupsState>(config.state_server_config)
        .await
        .map_err(|e| e.into())
}
