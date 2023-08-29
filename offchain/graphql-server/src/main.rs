// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use clap::Parser;

use graphql_server::{CLIConfig, GraphQLConfig};

use tracing::info;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: GraphQLConfig = CLIConfig::parse().into();

    logs::configure(&config.logs_config);

    info!(?config, "starting graphql server");
    graphql_server::run(config).await.map_err(|e| e.into())
}
