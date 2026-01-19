# restor-octo-giggle - Rust Server

The Rust server implementation for the restor-octo-giggle transport bus system, featuring protocol encapsulation, server initialization system, and message consumer capabilities.

## Project Overview

This project showcases a modular Rust application with:
- **Transport layer abstraction** with protocol encapsulation
- **Server initialization system** that maps transport protocols by configuration
- **Message consumer architecture** for processing incoming messages
- **TCP transport protocol** implementation (extensible for other protocols)
- **Protocol factory pattern** for dynamic protocol selection
- **Configuration-driven setup** with TCP as default protocol

## Key Features

- 🔌 **Protocol Encapsulation**: Transport protocols are fully encapsulated and interchangeable
- ⚙️ **Configuration-Based**: Initialize servers with protocol mapping via configuration
- 📨 **Message Consumer**: Built-in message consumption system with callback support
- 🔧 **Extensible Design**: Easy to add new transport protocols (UDP, WebSocket, etc.)
- 🚀 **Non-Blocking**: Server runs in background threads for concurrent message handling
- 🎯 **Type-Safe**: Leverages Rust's type system for safe protocol abstractions

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable version recommended)
  - Installation instructions: [rustup.rs](https://rustup.rs/)
  - Verify installation: `rustc --version`

## Getting Started

### Building the Project

To build the project, run:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

### Running the Application

To run the application:

```bash
cargo run
```

The server will start on `0.0.0.0:49152` by default with TCP protocol.

To test the server, you can use `telnet` or `nc` (netcat):

```bash
# In another terminal
telnet 127.0.0.1 49152
# or
nc 127.0.0.1 49152
```

Type messages and press Enter. The server will consume and process them.

## Project Structure

```
rust_samples/
├── Cargo.toml              # Project manifest and dependencies
├── Cargo.lock              # Dependency lock file
├── README.md               # This file
├── BUILD.md                # Cross-platform build guide
├── WINDOWS_NETWORKING.md   # Windows networking troubleshooting
├── Makefile                # Build commands
├── src/                    # Server source code
│   ├── main.rs             # Application entry point
│   ├── boot/
│   │   └── mod.rs          # Bootstrap initialization module
│   └── transport/
│       ├── mod.rs                          # Transport module exports
│       ├── bind_transaport.rs             # Transport binding functions
│       ├── transport_interface.rs         # Transport protocol trait definitions
│       ├── transport_server.rs            # Transport server implementation
│       ├── server.rs                      # Server with protocol encapsulation
│       ├── server_config.rs               # Server configuration (ProtocolType, ServerConfig)
│       ├── protocol_factory.rs            # Factory for creating protocol instances
│       ├── command_interface.rs           # Command interface traits
│       ├── protocols/                     # Protocol implementations
│       │   ├── mod.rs                     # Protocols module
│       │   └── tcp.rs                     # TCP transport protocol
│       └── servers/                       # Legacy server implementations
│           ├── mod.rs                     # Servers module
│           ├── server_interface.rs        # Legacy server trait definitions
│           ├── servers.rs                 # Legacy server implementations
│           └── tcp_server.rs              # Legacy TCP server
├── scripts/                # Build infrastructure
│   ├── build-all.sh        # Linux/macOS build script
│   ├── build-all.ps1       # Windows PowerShell build script
│   ├── build-all.bat       # Windows CMD build script
│   ├── package.sh          # Package distribution script
│   └── setup-cross.sh      # Cross-compilation setup
├── docker/                 # Docker infrastructure
│   ├── Dockerfile.server   # Multi-stage Docker build
│   └── docker-compose.yml  # Docker Compose configuration
├── .github/                # CI/CD infrastructure
│   └── workflows/
│       └── build.yml       # GitHub Actions workflow
└── target/                 # Build output directory (generated)
```

## Architecture

The project follows a modular, extensible architecture:

### Core Components

1. **Boot Module** (`boot/`)
   - Handles application initialization
   - Calls transport binding on startup

2. **Transport Layer** (`transport/`)
   - **TransportProtocol Trait**: Defines protocol interface (`listen()`, `protocol_name()`)
   - **TransportServer**: Manages protocol instances and message consumption
   - **Server**: High-level server with protocol encapsulation and initialization
   - **ProtocolFactory**: Maps protocol types to implementations
   - **ServerConfig**: Configuration structure for protocol and address settings

3. **Protocol Implementations** (`transport/protocols/`)
   - **TCP Transport**: Line-based message handling over TCP connections
   - Easily extensible for UDP, WebSocket, HTTP, etc.

4. **Message Consumer System**
   - Callback-based message processing
   - Type-safe consumer functions
   - Thread-safe message handling

### Architecture Flow

```
Application Startup
    ↓
boot::boot()
    ↓
bind_transport()
    ↓
Server::with_config() → ServerConfig (defaults to TCP)
    ↓
Server::initialize() → ProtocolFactory::create(ProtocolType::Tcp)
    ↓
TransportServer::new(protocol, address)
    ↓
Server::start(MessageConsumer)
    ↓
TransportProtocol::listen() → Spawns thread per connection
    ↓
MessageConsumer(message) → Process messages
```

## Usage Examples

### Basic Usage (Default TCP Configuration)

```rust
use crate::transport;

// Default: TCP on 0.0.0.0:49152
transport::bind_transport()?;
```

### Custom Address (Still TCP)

```rust
use crate::transport;

// Custom address with default TCP protocol
transport::bind_transport_with_address("0.0.0.0:9000".to_string())?;
```

### Full Configuration Control

```rust
use crate::transport::{bind_transport_with_config, ServerConfig, ProtocolType};

// Custom protocol and address
let config = ServerConfig::with_protocol(
    ProtocolType::Tcp,
    "127.0.0.1:8080".to_string()
);
bind_transport_with_config(config)?;
```

### Programmatic Server Initialization

```rust
use crate::transport::{Server, ServerConfig, MessageConsumer};

// Create server with custom configuration
let mut server = Server::with_config(
    ServerConfig::with_address("127.0.0.1:8080".to_string())
);

// Define custom message consumer
let consumer: MessageConsumer = Box::new(|message| {
    println!("Received: {}", message);
    // Your processing logic here
});

// Initialize and start
server.initialize_and_start(consumer)?;
```

### Custom Message Processing

```rust
use crate::transport::{bind_transport_with_config, ServerConfig, MessageConsumer};

let mut server = Server::new();

let consumer: MessageConsumer = Box::new(|message| {
    // Parse commands
    if message.starts_with("PING") {
        println!("Received PING command");
    }
    
    // Route to handlers
    match message.as_str() {
        "quit" => println!("Quit command received"),
        _ => println!("Processing: {}", message),
    }
});

server.initialize_and_start(consumer)?;
```

## Adding New Protocols

To add a new transport protocol (e.g., UDP):

1. **Implement the TransportProtocol trait**:
   ```rust
   // transport/protocols/udp.rs
   impl TransportProtocol for UdpTransport {
       fn listen(&self, address: &str, consumer: MessageConsumer) -> Result<()> {
           // UDP implementation
       }
       
       fn protocol_name(&self) -> &str {
           "UDP"
       }
   }
   ```

2. **Add to ProtocolType enum**:
   ```rust
   // transport/server_config.rs
   pub enum ProtocolType {
       Tcp,
       Udp,  // Add new variant
   }
   ```

3. **Update ProtocolFactory**:
   ```rust
   // transport/protocol_factory.rs
   match protocol_type {
       ProtocolType::Tcp => Ok(Arc::new(TcpTransport::new(address.to_string()))),
       ProtocolType::Udp => Ok(Arc::new(UdpTransport::new(address.to_string()))),
   }
   ```

4. **Register in protocols module**:
   ```rust
   // transport/protocols/mod.rs
   pub mod udp;
   pub use udp::UdpTransport;
   ```

## Dependencies

- **once_cell** (^1.19) - Thread-safe initialization of static values

## Configuration

### Default Settings

- **Protocol**: TCP
- **Address**: `0.0.0.0:49152`

### Protocol Types

- `ProtocolType::Tcp` - TCP transport protocol (default)
- Future protocols can be added via the `ProtocolType` enum

### ServerConfig Options

- `ServerConfig::new()` - Creates config with defaults (TCP on 0.0.0.0:49152)
- `ServerConfig::with_address(address)` - Custom address, default protocol
- `ServerConfig::with_protocol(protocol, address)` - Full customization
- Builder methods: `set_protocol()`, `set_address()`

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Building Documentation

```bash
cargo doc --open
```

## Infrastructure & CI/CD

This directory contains all build infrastructure and CI/CD pipelines for the restor-octo-giggle bus server:

### Build Scripts

- **`scripts/build-all.sh`** - Build for all platforms (Linux/macOS)
- **`scripts/build-all.ps1`** - Build for all platforms (Windows PowerShell)
- **`scripts/build-all.bat`** - Build for all platforms (Windows CMD)
- **`scripts/package.sh`** - Create distribution packages
- **`scripts/setup-cross.sh`** - Setup cross-compilation tooling

### Docker

- **`docker/Dockerfile.server`** - Multi-stage Docker build for the server
- **`docker/docker-compose.yml`** - Docker Compose configuration

### CI/CD

- **`.github/workflows/build.yml`** - GitHub Actions workflow for automated builds

### Documentation

- **`BUILD.md`** - Comprehensive cross-platform build guide
- **`WINDOWS_NETWORKING.md`** - Windows networking troubleshooting

### Build Commands

Use the Makefile for convenient build commands:

```bash
make build          # Build for current platform
make build-all      # Build for all platforms
make docker-build   # Build Docker image
make docker-run     # Run in Docker
make package        # Create distribution packages
```

## Message Format

The TCP transport protocol expects line-delimited messages:
- Each message should end with a newline (`\n`)
- Empty lines are ignored
- Messages are processed asynchronously in separate threads

Example:
```
Hello, Server!
PING
GET /status
```

## Threading Model

- Main thread: Handles application lifecycle and user input
- Server thread: Listens for incoming connections
- Client threads: One thread per client connection for message processing

## Windows Networking

If you encounter socket access errors on Windows, see [WINDOWS_NETWORKING.md](WINDOWS_NETWORKING.md) for troubleshooting.

## Version

Current version: **0.1.0**

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Future Enhancements

- [ ] UDP protocol implementation
- [ ] WebSocket protocol support
- [ ] HTTP/HTTPS transport protocols
- [ ] Message routing and command parsing
- [ ] Response sending capabilities
- [ ] Connection pooling
- [ ] SSL/TLS support
- [ ] Configuration file support (TOML/YAML)
