# restor-octo-giggle - Core Transport Server

High-performance transport bus server written in Rust. This is the core application that receives messages from producers and routes them to registered consumers based on event names.

## Features

- ✅ **Multi-Protocol Support** - TCP (extensible for future protocols)
- ✅ **Event-Based Routing** - Route messages by event name
- ✅ **Producer Pool** - Manage multiple external consumers
- ✅ **Registration Server** - Runtime consumer registration
- ✅ **Observer Pattern** - Event subscription system
- ✅ **Cross-Platform** - Windows, Linux, macOS support

## Quick Start

```bash
# Build
cargo build

# Run
cargo run

# Run test consumer server
cargo run --bin test_consumer_server
```

## Architecture

```
┌─────────────┐
│   Producer  │ ──TCP──> ┌──────────────┐
│  (JS/Any)   │          │ Rust Server  │ ──TCP──> ┌─────────────┐
└─────────────┘          │  (Consumer)  │          │  Consumer   │
                         └──────────────┘          │  (Producer)  │
                              │                      └─────────────┘
                              │
                              ▼
                         ┌──────────────┐
                         │ Event Router │
                         └──────────────┘
```

## Documentation

All documentation is in the `docs/` directory:

- [README](docs/README.md) - Detailed documentation
- [Build Guide](docs/BUILD.md) - Cross-platform build instructions
- [Windows Networking](docs/WINDOWS_NETWORKING.md) - Windows-specific setup
- [Producer Pool](docs/PRODUCER_POOL.md) - Producer pool system
- [Event Routing](docs/EVENT_ROUTING.md) - Event-based message routing
- [Integration Testing](docs/INTEGRATION_TESTING.md) - Testing guide
- [Traits Explained](docs/TRAITS_EXPLAINED.md) - Rust traits documentation

## Configuration

### Environment Variables

```bash
# Server address (default: 0.0.0.0:49152)
TRANSPORT_ADDRESS=0.0.0.0:49152

# Producer addresses (optional)
PRODUCER_CONSUMER1=tcp://127.0.0.1:9000
PRODUCER_CONSUMER2=tcp://127.0.0.1:9001
```

## Message Formats

### JSON Format
```json
{
    "msg": "Message content",
    "event_name": "event_name"
}
```

### Simple Format
```
event_name:message content
```

## License

MIT
