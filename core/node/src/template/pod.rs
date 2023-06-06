use async_trait::async_trait;
use draco_interfaces::{
    common::WithStartAndShutdown, config::ConfigConsumer, pod::DeliveryAcknowledgmentSocket,
    signer::SubmitTxSocket, DeliveryAcknowledgmentAggregatorInterface,
};

use super::config::Config;

#[derive(Clone)]
pub struct DeliveryAcknowledgmentAggregator {}

#[async_trait]
impl WithStartAndShutdown for DeliveryAcknowledgmentAggregator {
    /// Returns true if this system is running or not.
    fn is_running(&self) -> bool {
        todo!()
    }

    /// Start the system, should not do anything if the system is already
    /// started.
    async fn start(&self) {
        todo!()
    }

    /// Send the shutdown signal to the system.
    async fn shutdown(&self) {
        todo!()
    }
}

#[async_trait]
impl DeliveryAcknowledgmentAggregatorInterface for DeliveryAcknowledgmentAggregator {
    /// Initialize a new delivery acknowledgment aggregator.
    async fn init(_config: Self::Config, _submit_tx: SubmitTxSocket) -> anyhow::Result<Self> {
        todo!()
    }

    /// Returns the socket that can be used to submit delivery acknowledgments to be aggregated.
    fn socket(&self) -> DeliveryAcknowledgmentSocket {
        todo!()
    }
}

impl ConfigConsumer for DeliveryAcknowledgmentAggregator {
    const KEY: &'static str = "pod";

    type Config = Config;
}