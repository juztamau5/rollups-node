// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use eth_block_history::{BlockArchive, BlockSubscriber, SubscriptionError};
use eth_state_fold::StateFoldEnvironment;
use eth_state_fold_types::BlockStreamItem;
use ethers::providers::{
    Http, HttpRateLimitRetryPolicy, Provider, RetryClient,
};
use rollups_events::DAppMetadata;
use snafu::ResultExt;
use std::sync::{Arc, Mutex};
use tokio_stream::Stream;
use types::UserData;
use url::Url;

use crate::error::ParseSnafu;
use crate::{
    config::EthInputReaderConfig,
    error::{BrokerSnafu, EthInputReaderError},
    machine::{BrokerStatus, Context},
    metrics::EthInputReaderMetrics,
};

pub type InputProvider = Provider<RetryClient<Http>>;

pub async fn create_provider(
    config: &EthInputReaderConfig,
) -> Result<Arc<InputProvider>, EthInputReaderError> {
    let http = Http::new(
        Url::parse(&config.bh_config.http_endpoint).context(ParseSnafu)?,
    );

    let retry_client =
        RetryClient::new(http, Box::new(HttpRateLimitRetryPolicy), 10, 1000);

    let provider = Provider::new(retry_client);

    Ok(Arc::new(provider))
}

pub async fn create_block_subscriber(
    config: &EthInputReaderConfig,
    provider: Arc<InputProvider>,
) -> Result<Arc<BlockSubscriber<InputProvider>>, EthInputReaderError> {
    Ok(Arc::new(
        BlockSubscriber::start(
            Arc::clone(&provider),
            config.bh_config.ws_endpoint.to_owned(),
            config.bh_config.block_timeout,
            config.bh_config.max_depth,
        )
        .await
        .expect("should create block history"),
    ))
}

pub async fn create_subscription(
    block_subscriber: Arc<BlockSubscriber<InputProvider>>,
) -> Result<
    impl Stream<Item = Result<BlockStreamItem, SubscriptionError<InputProvider>>>
        + std::marker::Unpin,
    EthInputReaderError,
> {
    Ok(block_subscriber
        .subscribe_new_blocks_at_depth(10)
        .await
        .expect("should subscribe new blocks"))
}

pub async fn create_env(
    config: &EthInputReaderConfig,
    provider: Arc<InputProvider>,
    block_archive: Arc<BlockArchive<InputProvider>>,
) -> Result<
    Arc<StateFoldEnvironment<InputProvider, Mutex<UserData>>>,
    EthInputReaderError,
> {
    let env = StateFoldEnvironment::new(
        provider,
        Some(block_archive),
        config.sf_config.safety_margin,
        config.sf_config.genesis_block,
        config.sf_config.query_limit_error_codes.clone(),
        config.sf_config.concurrent_events_fetch,
        10000,
        Mutex::new(UserData::default()),
    );

    Ok(Arc::new(env))
}

pub async fn create_context(
    config: &EthInputReaderConfig,
    block_subscriber: Arc<BlockSubscriber<InputProvider>>,
    broker: &impl BrokerStatus,
    dapp_metadata: DAppMetadata,
    metrics: EthInputReaderMetrics,
) -> Result<Context, EthInputReaderError> {
    let genesis_timestamp: u64 = block_subscriber
        .block_archive
        .block_with_hash(&config.dapp_deployment.deploy_block_hash)
        .await
        .expect("should get genesis block")
        .timestamp
        .as_u64();
    let epoch_length = config.epoch_duration;
    let context = Context::new(
        genesis_timestamp,
        epoch_length,
        broker,
        dapp_metadata,
        metrics,
    )
    .await
    .context(BrokerSnafu)?;

    Ok(context)
}
