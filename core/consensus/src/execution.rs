use std::sync::Arc;

use async_trait::async_trait;
use draco_interfaces::{
    types::{Block, UpdateRequest},
    ExecutionEngineSocket,
};
use narwhal_executor::ExecutionState;
use narwhal_types::{Batch, BatchAPI, ConsensusOutput};
use tokio::sync::Notify;

pub struct Execution {
    /// Managing certificates generated by narwhal.
    executor: ExecutionEngineSocket,
    reconfigure_notify: Arc<Notify>,
}

impl Execution {
    pub fn new(executor: ExecutionEngineSocket, reconfigure_notify: Arc<Notify>) -> Self {
        Self {
            executor,
            reconfigure_notify,
        }
    }

    async fn submit_batch(&self, batch: Vec<Batch>) {
        let mut change_epoch = false;
        for batch in batch {
            let block = Block {
                transactions: batch
                    .transactions()
                    .iter()
                    .filter_map(|txn| bincode::deserialize::<UpdateRequest>(txn).ok())
                    .collect(),
            };

            // Unfailable
            let results = self.executor.run(block).await.unwrap();

            if results.change_epoch {
                change_epoch = true;
            }
        }
        if change_epoch {
            self.reconfigure_notify.notify_waiters();
        }
    }
}

#[async_trait]
impl ExecutionState for Execution {
    async fn handle_consensus_output(&self, consensus_output: ConsensusOutput) {
        for (_, batches) in consensus_output.batches {
            self.submit_batch(batches).await
        }
    }

    async fn last_executed_sub_dag_index(&self) -> u64 {
        0
    }
}