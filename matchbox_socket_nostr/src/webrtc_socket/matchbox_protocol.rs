use derive_more::From;
use serde::{Deserialize, Serialize};

/// A unique identifier for a peer in the network, based on a Nostr public key.
///
/// This struct represents a peer's identity using an x-only public key from the secp256k1
/// elliptic curve, which is compatible with Schnorr signatures used in Nostr.
#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, From, Hash, PartialOrd, Ord,
)]
pub struct PeerId(pub nostr::secp256k1::XOnlyPublicKey);

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
