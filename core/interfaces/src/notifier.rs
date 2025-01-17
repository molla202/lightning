use std::time::Duration;

use infusion::{c, ok};
use tokio::sync::mpsc;

use crate::infu_collection::Collection;
use crate::ApplicationInterface;

#[derive(Debug)]
pub enum Notification {
    NewEpoch,
    BeforeEpochChange,
}

#[infusion::service]
pub trait NotifierInterface<C: Collection>: Sync + Send + Clone {
    fn _init(app: ::ApplicationInterface) {
        ok!(Self::init(app))
    }

    fn init(app: &c!(C::ApplicationInterface)) -> Self;

    fn notify_on_new_epoch(&self, tx: mpsc::Sender<Notification>);

    fn notify_before_epoch_change(&self, duration: Duration, tx: mpsc::Sender<Notification>);
}
