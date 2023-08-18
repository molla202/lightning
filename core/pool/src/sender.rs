use std::marker::PhantomData;

use async_trait::async_trait;
use fleek_crypto::NodePublicKey;
use lightning_interfaces::{schema::LightningMessage, SenderInterface};

pub struct Sender<T> {
    _marker: PhantomData<T>,
}

#[async_trait]
impl<T> SenderInterface<T> for Sender<T>
where
    T: LightningMessage,
{
    fn pk(&self) -> &NodePublicKey {
        todo!()
    }

    async fn send(&self, msg: T) -> bool {
        todo!()
    }
}