use clap::Parser;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[derive(Clone, Parser, Debug)]
#[command(name = "log_config")]
#[command(about = "Configuration for Logs")]
pub struct LogsEnvCliConfig {
    #[arg(long, env, default_value = "false")]
    pub logs_enable_timestamp: bool,

    #[arg(long, env, default_value = "false")]
    pub logs_enable_color: bool,
}

#[derive(Clone, Debug, Default)]
pub struct LogsConfig {
    pub enable_timestamp: bool,
    pub enable_color: bool,
}

impl LogsConfig {
    pub fn initialize(env_cli_config: LogsEnvCliConfig) -> Self {
        let enable_timestamp = env_cli_config.logs_enable_timestamp;
        let enable_color = env_cli_config.logs_enable_color;

        LogsConfig {
            enable_timestamp,
            enable_color,
        }
    }
}

impl From<LogsEnvCliConfig> for LogsConfig {
    fn from(cli_config: LogsEnvCliConfig) -> LogsConfig {
        LogsConfig::initialize(cli_config)
    }
}

pub fn configure(config: &LogsConfig) {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let subscribe_builder = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(filter)
        .with_ansi(config.enable_color);

    if !config.enable_timestamp {
        subscribe_builder.without_time().init();
    } else {
        subscribe_builder.init();
    }
}
