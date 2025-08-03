# bevy_matchbox_nostr

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Bevy](https://img.shields.io/badge/bevy-0.15-blue.svg)](https://bevyengine.org)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)

**Decentralized peer-to-peer WebRTC networking for Bevy games using Nostr as the signaling layer.**

This project replaces traditional centralized signaling servers with the [Nostr protocol](https://nostr.org), enabling truly decentralized multiplayer gaming without relying on any central infrastructure.

## What is Nostr?

[Nostr](https://nostr.org) (Notes and Other Stuff Transmitted by Relays) is a simple, open protocol that enables censorship-resistant global "social" networks. Instead of relying on centralized servers, Nostr uses a network of decentralized relays to transmit messages between users, each identified by cryptographic key pairs.

## How it Works

Traditional WebRTC requires a signaling server to help peers discover each other and establish connections. This project replaces that centralized signaling server with Nostr's decentralized relay network:

```
┌─────────────┐    Nostr Relays    ┌─────────────┐
│   Player A  │ ◄─────────────────► │   Player B  │
│             │                    │             │
│ ┌─────────┐ │                    │ ┌─────────┐ │
│ │ Nostr   │ │ ◄─── Encrypted ───► │ │ Nostr   │ │
│ │ Keys    │ │      Signaling      │ │ Keys    │ │
│ └─────────┘ │                    │ └─────────┘ │
└─────────────┘                    └─────────────┘
       │                                  │
       └────── Direct WebRTC P2P ─────────┘
```

1. **Peer Discovery**: Players connect to one or more Nostr relays
2. **Signaling**: WebRTC offers/answers are encrypted using [NIP-04](https://nostr.org/specs/nip-04) and sent as Nostr events
3. **Connection**: Once WebRTC negotiation completes, data flows directly between peers
4. **Decentralization**: No single point of failure - if one relay goes down, others continue working

## Features

- ✅ **Truly Decentralized**: No central servers required
- ✅ **Censorship Resistant**: Uses Nostr's distributed relay network  
- ✅ **Encrypted Signaling**: All signaling data encrypted with NIP-04
- ✅ **Bevy 0.15 Compatible**: Works with the latest Bevy release
- ✅ **Cross-Platform**: Supports both native and WASM targets
- ✅ **GGRS Compatible**: Drop-in replacement for rollback netcode
- ✅ **Multiple Channels**: Configurable reliable/unreliable data channels

## Quick Start

### Add to Cargo.toml

```toml
[dependencies]
bevy = "0.15"
bevy_matchbox_nostr = "0.6.4"
nostr = { version = "0.43", features = ["nip04"] }
```

### Basic Usage

```rust
use bevy::prelude::*;
use bevy_matchbox_nostr::prelude::*;
use nostr::Keys;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start_matchbox_socket)
        .add_systems(Update, handle_socket_events)
        .run();
}

fn start_matchbox_socket(mut commands: Commands) {
    // Generate or load Nostr keys
    let nostr_keys = Keys::generate();
    
    // Create socket with Nostr relay as room URL
    let socket = MatchboxSocket::new_reliable(
        "wss://relay.damus.io/your-game-room", 
        nostr_keys
    );
    
    commands.insert_resource(socket);
}

fn handle_socket_events(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    // Handle incoming connections
    for (peer, new_state) in socket.update_peers() {
        match new_state {
            PeerState::Connected => {
                info!("Peer {peer:?} connected");
            }
            PeerState::Disconnected => {
                info!("Peer {peer:?} disconnected");
            }
        }
    }
    
    // Send data to all connected peers
    let packet = b"Hello, decentralized world!";
    socket.send(packet.into());
    
    // Receive data from peers
    for (peer, packet) in socket.receive() {
        info!("Received from {peer:?}: {:?}", packet);
    }
}
```

### GGRS Integration

For rollback netcode with GGRS:

```rust
use bevy_matchbox_nostr::prelude::*;
use bevy_ggrs::prelude::*;

fn start_ggrs_socket(mut commands: Commands) {
    let nostr_keys = Keys::generate();
    let socket = MatchboxSocket::new_ggrs(
        "wss://relay.damus.io/fighting-game-room",
        nostr_keys
    );
    commands.insert_resource(socket);
}
```

## Project Structure

This repository contains two main crates:

- **`matchbox_socket_nostr`**: Core WebRTC socket implementation with Nostr signaling
- **`bevy_matchbox_nostr`**: Bevy integration providing Components, Resources, and Commands

## Nostr Relay Requirements

Any Nostr relay that supports [NIP-04](https://nostr.org/specs/nip-04) encrypted direct messages will work. Popular public relays include:

- `wss://relay.damus.io`
- `wss://nos.lol` 
- `wss://relay.snort.social`
- `wss://nostr.wine`

For production games, consider running your own relay or using multiple relays for redundancy.

## Platform Support

### Native (Desktop/Mobile)
- **Linux**: ✅ Full support
- **Windows**: ✅ Full support  
- **macOS**: ✅ Requires LLVM (see build notes below)
- **iOS/Android**: ✅ Via Bevy mobile support

### Web (WASM)
- ✅ All modern browsers with WebRTC support
- ✅ Works with `wasm-bindgen` and `bevy_web`

### Build Notes for macOS

macOS requires LLVM for cryptographic compilation:

```bash
brew install llvm
export LLVM_PATH=$(brew --prefix llvm)
export AR="${LLVM_PATH}/bin/llvm-ar"
export CC="${LLVM_PATH}/bin/clang"

# For WASM target
cargo build --target wasm32-unknown-unknown
```

## Migration from Centralized Matchbox

Migrating from the original matchbox is straightforward:

```rust
// Before: Centralized matchbox
let socket = MatchboxSocket::new_reliable("wss://matchbox.example.com/room");

// After: Decentralized with Nostr
let nostr_keys = Keys::generate(); // or load existing keys
let socket = MatchboxSocket::new_reliable("wss://relay.damus.io/room", nostr_keys);
```

The socket API remains identical - only the construction changes.

## API Compatibility

### Bevy Version Support
- **Current**: Bevy 0.15.x ✅
- **Previous**: Bevy 0.14.x ❌ (use earlier versions)

### Nostr Version Support  
- **Current**: nostr 0.43.x with NIP-04 support ✅
- **Previous**: nostr 0.21.x ❌ (significant API changes)

## Security Considerations

### Key Management
- **Development**: Use `Keys::generate()` for testing
- **Production**: Implement proper key storage and recovery
- **Privacy**: Each game session can use different keys

### Relay Trust
- Relays only see encrypted signaling data (via NIP-04)
- Game data flows directly between peers via WebRTC
- Use multiple relays to avoid single points of failure

### Network Security
- All signaling encrypted with NIP-04 standard
- WebRTC provides end-to-end encryption for game data
- Peer authentication via Nostr cryptographic identities

## Contributing

Contributions welcome! This project follows the same patterns as the original Matchbox:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable  
5. Submit a pull request

### Development Setup

```bash
git clone https://github.com/your-username/matchbox_nostr.git
cd matchbox_nostr
cargo test
cargo build
```

## Roadmap

- [ ] **Examples**: Complete working examples with different game types
- [ ] **Relay Discovery**: Automatic relay selection and failover
- [ ] **Key Management**: Integrated key storage solutions
- [ ] **Performance**: Benchmarks vs centralized signaling
- [ ] **Mobile**: Enhanced mobile platform support
- [ ] **Nostr Extensions**: Support for additional NIPs (Nostr Improvement Proposals)

## Related Projects

- **[Matchbox](https://github.com/johanhelsing/matchbox)**: Original centralized WebRTC library for Bevy
- **[GGRS](https://github.com/gschup/ggrs)**: Rollback netcode library with Bevy integration
- **[Nostr Protocol](https://nostr.org)**: Decentralized communication protocol specification
- **[rust-nostr](https://rust-nostr.org)**: Rust implementation of Nostr protocol

## Acknowledgments

- **[Johan Helsing](https://github.com/johanhelsing)** for the original Matchbox library
- **[Ernest Wong](https://github.com/ErnWong)** for the Dango Tribute WebRTC experiment
- **Nostr Community** for building the decentralized communication protocol
- **Bevy Community** for the amazing game engine

## License

This project is dual-licensed under:

- [MIT License](LICENSE-MIT) or <http://opensource.org/licenses/MIT>
- [Apache License, Version 2.0](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

**Learn more about Nostr**: [nostr.org](https://nostr.org) | **Learn more about Bevy**: [bevyengine.org](https://bevyengine.org)