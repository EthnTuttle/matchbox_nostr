#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod error;
#[cfg(feature = "ggrs")]
mod ggrs_socket;
mod webrtc_socket;

pub use error::Error;

pub use webrtc_socket::{
    error::GetChannelError, BuildablePlurality, ChannelConfig, ChannelPlurality, MessageLoopFuture,
    MultipleChannels, NoChannels, Packet, PeerId, PeerState, RtcIceServerConfig, SingleChannel,
    WebRtcChannel, WebRtcSocket, WebRtcSocketBuilder,
};
