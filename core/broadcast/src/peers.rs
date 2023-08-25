use std::pin::Pin;

use fleek_crypto::NodePublicKey;
use futures::stream::FuturesUnordered;
use futures::Future;
use fxhash::{FxHashMap, FxHashSet};
use lightning_interfaces::types::NodeIndex;
use lightning_interfaces::{ReceiverInterface, SenderInterface};

use crate::ev::Topology;
use crate::frame::{Digest, Frame};
use crate::receivers::Receivers;
use crate::MessageInternedId;

/// This struct is responsible for holding the state of the current peers
/// that we are connected to.
pub struct Peers<S: SenderInterface<Frame>, R: ReceiverInterface<Frame>> {
    /// Map each public key to the info we have about that peer.
    peers: FxHashMap<NodePublicKey, Peer<S>>,
    /// The message queue from all the connections we have.
    incoming_messages: Receivers<R>,
}

/// An interned id. But not from our interned table.
type RemoteInternedId = MessageInternedId;

struct Peer<S> {
    /// The index of the node.
    index: NodeIndex,
    /// The sender that we can use to send messages to this peer.
    sender: S,
    /// The origin of this connection can tell us if we started this connection or if
    /// the remote has dialed us.
    origin: ConnectionOrigin,
    /// The status of our connection with this peer.
    status: ConnectionStatus,
    /// The stats we have from this connection to the peer.
    stats: ConnectionStats,
    // We know this peer has these messages. They have advertised them to us before.
    //
    // Here we are storing the mapping from the interned id of a message in our table,
    // to the interned id of the message known to the client.
    has: FxHashMap<MessageInternedId, RemoteInternedId>,
}

enum ConnectionOrigin {
    // We have established the connection.
    Us,
    /// The remote has dialed us and we have this connection because we got
    /// a connection from the listener.
    Remote,
}

pub enum ConnectionStatus {
    /// The connection with the other peer is open.
    ///
    /// We are sending advertisements to this node and actively listening for their
    /// advertisements.
    Open,
    /// The connection with this node is closing. We do not wish to interact with it
    /// anymore. But since we may have pending communication with them. We are still
    /// keeping the connection alive.
    ///
    /// At this point we do not care about their advertisements. We only care about
    /// the messages they owe us. Once the other peer does not owe us anything anymore
    /// we close the connection.
    Closing,
    /// The connection with the other peer is closed and we are not communicating with
    /// the node.
    Closed,
}

/// A bunch of statistics that we gather from a peer throughout the life of the gossip.
pub struct ConnectionStats {
    /// How many things have we advertised to this node.
    advertisements_received_from_us: usize,
    /// How many things has this peer advertised to us.
    advertisements_received_from_peer: usize,
    /// How many `WANT`s have we sent to this node.
    wants_received_from_us: usize,
    /// How many `WANT`s has this peer sent our way.
    wants_received_from_peer: usize,
    /// Valid messages sent by this node to us.
    messages_received_from_peer: usize,
    /// Number of messages we have received from this peer that
    /// we did not continue propagating.
    invalid_messages_received_from_peer: usize,
}

impl<S: SenderInterface<Frame>, R: ReceiverInterface<Frame>> Peers<S, R> {
    /// Apply a new topology and update the connection graph.
    pub fn apply_topology(&mut self, new_topology: Topology) {}

    /// Advertise the given digest to every peer. This method does not perform
    /// the advertisements with the peers if we know they already have the digest.
    ///
    /// That is when they have previously advertised the same digest to us.
    pub fn advertise(&self, _digest: Digest) {}

    pub async fn handle_new_incoming_connection(&mut self, (sender, receiver): (S, R)) {
        // let mut set = FuturesUnordered::new();
        // set.push(receiver.recv());
    }
}

impl<S: SenderInterface<Frame>, R: ReceiverInterface<Frame>> Default for Peers<S, R> {
    fn default() -> Self {
        Self {
            peers: Default::default(),
            incoming_messages: Default::default(),
        }
    }
}