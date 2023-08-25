// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use clap::Parser;

use logs::{LogsConfig, LogsEnvCliConfig};
pub use rollups_data::{RepositoryCLIConfig, RepositoryConfig};
pub use rollups_events::{
    BrokerCLIConfig, BrokerConfig, DAppMetadata, DAppMetadataCLIConfig,
};

#[derive(Debug)]
pub struct IndexerConfig {
    pub repository_config: RepositoryConfig,
    pub dapp_metadata: DAppMetadata,
    pub broker_config: BrokerConfig,
    pub logs_config: LogsConfig,
    pub healthcheck_port: u16,
}

#[derive(Parser)]
pub struct CLIConfig {
    #[command(flatten)]
    repository_config: RepositoryCLIConfig,

    #[command(flatten)]
    dapp_metadata_config: DAppMetadataCLIConfig,

    #[command(flatten)]
    broker_config: BrokerCLIConfig,

    #[command(flatten)]
    pub logs_config: LogsEnvCliConfig,

    /// Port of health check
    #[arg(
        long = "healthcheck-port",
        env = "INDEXER_HEALTHCHECK_PORT",
        default_value_t = 8080
    )]
    pub healthcheck_port: u16,
}

impl From<CLIConfig> for IndexerConfig {
    fn from(cli_config: CLIConfig) -> Self {
        Self {
            repository_config: cli_config.repository_config.into(),
            dapp_metadata: cli_config.dapp_metadata_config.into(),
            broker_config: cli_config.broker_config.into(),
            logs_config: cli_config.logs_config.into(),
            healthcheck_port: cli_config.healthcheck_port,
        }
    }
}
