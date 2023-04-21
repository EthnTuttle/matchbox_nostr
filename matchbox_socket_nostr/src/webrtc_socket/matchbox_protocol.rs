use derive_more::From;
use nostr::secp256k1::XOnlyPublicKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, From, Hash, PartialOrd, Ord,
)]
pub struct PeerId(pub XOnlyPublicKey);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PeerRequest<S> {
    Signal { receiver: PeerId, data: S },
    KeepAlive,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PeerEvent<S> {
    /// Sent by the server to the connecting peer, immediately after connection
    /// before any other events
    IdAssigned(PeerId),
    NewPeer(PeerId),
    PeerLeft(PeerId),
    Signal {
        sender: PeerId,
        data: S,
    },
}
