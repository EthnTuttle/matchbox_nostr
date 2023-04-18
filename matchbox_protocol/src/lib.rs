use cfg_if::cfg_if;
use derive_more::From;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

// If only "json" feature is enabled
#[cfg(all(feature = "json", not(feature = "nostr")))]
#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, From, Hash, PartialOrd, Ord,
)]
pub struct PeerId(pub Uuid);

// If only "nostr" feature is enabled
#[cfg(all(feature = "nostr", not(feature = "json")))]
use nostr::secp256k1::XOnlyPublicKey;

#[cfg(all(feature = "nostr", not(feature = "json")))]
#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, From, Hash, PartialOrd, Ord,
)]
pub struct PeerId(pub XOnlyPublicKey);

// If neither "nostr" nor "json" feature is enabled
#[cfg(all(not(feature = "nostr"), not(feature = "json")))]
#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, From, Hash, PartialOrd, Ord,
)]
pub struct PeerId(pub Uuid);

/// Requests go from peer to signaling server
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PeerRequest<S> {
    Signal { receiver: PeerId, data: S },
    KeepAlive,
}

/// Events go from signaling server to peer
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

cfg_if! {
    if #[cfg(feature = "json")] {
        pub type JsonPeerRequest = PeerRequest<serde_json::Value>;
        pub type JsonPeerEvent = PeerEvent<serde_json::Value>;


        impl ToString for JsonPeerRequest {
            fn to_string(&self) -> String {
                serde_json::to_string(self).expect("error serializing message")
            }
        }
        impl std::str::FromStr for JsonPeerRequest {
            type Err = serde_json::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                serde_json::from_str(s)
            }
        }

        impl ToString for JsonPeerEvent {
            fn to_string(&self) -> String {
                serde_json::to_string(self).expect("error serializing message")
            }
        }
        impl std::str::FromStr for JsonPeerEvent {
            type Err = serde_json::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                serde_json::from_str(s)
            }
        }
    }
}
