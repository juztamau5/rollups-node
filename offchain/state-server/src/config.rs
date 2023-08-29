use clap::Parser;
use eth_state_server_lib::config::{
    Result, StateServerConfig, StateServerEnvCLIConfig,
};
use logs::{LogsConfig, LogsEnvCliConfig};

#[derive(Debug, Clone, Parser)]
#[command(name = "state_server_config")]
#[command(about = "Configuration for the state-server")]
pub struct EnvCLIConfig {
    #[command(flatten)]
    pub state_server_config: StateServerEnvCLIConfig,

    #[command(flatten)]
    pub logs_config: LogsEnvCliConfig,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub state_server_config: StateServerConfig,
    pub logs_config: LogsConfig,
}

impl Config {
    pub fn initialize(env_cli_config: EnvCLIConfig) -> Result<Self> {
        let state_server_config =
            StateServerConfig::initialize(env_cli_config.state_server_config);
        let logs_config = LogsConfig::initialize(env_cli_config.logs_config);

        Ok(Self {
            state_server_config: state_server_config?,
            logs_config: logs_config,
        })
    }

    pub fn initialize_from_args() -> Result<Self> {
        let env_cli_config = EnvCLIConfig::parse();
        Self::initialize(env_cli_config)
    }
}
