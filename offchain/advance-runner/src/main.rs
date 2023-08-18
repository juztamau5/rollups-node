// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use advance_runner::config::AdvanceRunnerConfig;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AdvanceRunnerConfig::parse()?;

    logs::configure(&config.logs_config);

    info!(?config, "Starting Advance Runner");
    advance_runner::run(config).await.map_err(|e| e.into())
}
