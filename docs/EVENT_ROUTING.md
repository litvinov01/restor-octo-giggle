# Event-Based Message Routing System

## Architecture Overview

The system implements an event-based message routing pattern where:

```
ext_producer -> someprotocol://uri (message) -> { msg, event_name } -> 
  spread msg for all observable for such event producers -> 
  someprotocol://uri (message) -> ext_consumer
```

## Components

### 1. Registration Server (`registration_server.rs`)

A root TCP server (port 49153 by default) that allows external consumers to register at runtime.

**Commands:**
- `REGISTER <id> <protocol>://<address> [events...]` - Register a consumer with optional event subscriptions
- `SUBSCRIBE <id> <event_name>` - Subscribe a producer to an event
- `UNSUBSCRIBE <id> <event_name>` - Unsubscribe a producer from an event
- `LIST` - List all registered producers and events
- `QUIT` - Close connection

**Example:**
```bash
telnet localhost 49153
REGISTER consumer1 tcp://127.0.0.1:9000 event1 event2
SUBSCRIBE consumer1 event3
LIST
```

### 2. Observer Pattern (`producers/observer.rs`)

Producers act as observers that can subscribe to events:

```rust
// Producer implements Observer trait
pub trait Observer: Send + Sync {
    fn id(&self) -> &str;
    fn notify(&self, message: &str) -> std::io::Result<()>;
    fn subscribed_events(&self) -> Vec<String>;
}
```

### 3. Event-Based Producer Pool (`producers/producer_pool.rs`)

The producer pool maintains:
- **Producer registry**: Map of producer ID -> Producer
- **Event subscriptions**: Map of event_name -> Set of producer IDs

**Key Methods:**
- `add_producer_with_events()` - Add producer with initial event subscriptions
- `subscribe_producer_to_event()` - Subscribe producer to event
- `forward_to_event()` - Forward message to all producers subscribed to event
- `get_event_subscribers()` - Get producers subscribed to an event

### 4. Event Message Format (`message.rs`)

Messages can be in two formats:

**JSON Format:**
```json
{"msg": "Hello World", "event_name": "user_message"}
```

**Simple Format:**
```
event_name:Hello World
```

The system automatically parses both formats.

### 5. Message Consumer Integration (`bind_transaport.rs`)

The message consumer:
1. Receives messages from external producers
2. Parses message to extract `{msg, event_name}`
3. Routes message to all producers subscribed to that event
4. Each producer forwards to its external consumer

## Usage Flow

### 1. Start the Server

```bash
cargo run
```

This starts:
- **Transport server** on port 49152 (receives messages)
- **Registration server** on port 49153 (registers consumers)

### 2. Register External Consumers

```bash
# Connect to registration server
telnet localhost 49153

# Register consumer1 for events "user_message" and "system_log"
REGISTER consumer1 tcp://127.0.0.1:9000 user_message system_log

# Register consumer2 for event "user_message"
REGISTER consumer2 tcp://127.0.0.1:9001 user_message
```

### 3. Send Messages from External Producer

```bash
# Connect to transport server
telnet localhost 49152

# Send message in simple format
user_message:Hello from producer!

# Or send JSON format
{"msg": "Hello from producer!", "event_name": "user_message"}
```

### 4. Message Routing

When a message is received:
1. System parses `event_name` (e.g., "user_message")
2. Finds all producers subscribed to "user_message"
3. Forwards message to each producer
4. Each producer sends to its external consumer

**Result:**
- `consumer1` (subscribed to "user_message") receives: "Hello from producer!"
- `consumer2` (subscribed to "user_message") receives: "Hello from producer!"

## Example Scenario

```
┌─────────────┐
│   Producer  │──> tcp://localhost:49152
│  (External) │    Message: "user_message:Hello"
└─────────────┘

         │
         ▼
┌─────────────────────┐
│  Transport Server   │
│  (Port 49152)       │
└─────────────────────┘
         │
         ▼ Parse: event_name="user_message", msg="Hello"
┌─────────────────────┐
│  Event Router       │
└─────────────────────┘
         │
         ├──> consumer1 (subscribed to "user_message")
         │    └──> tcp://127.0.0.1:9000
         │
         └──> consumer2 (subscribed to "user_message")
              └──> tcp://127.0.0.1:9001
```

## Dynamic Registration

Consumers can be registered at runtime:

```bash
# Register new consumer
REGISTER consumer3 tcp://127.0.0.1:9002 user_message

# Subscribe to additional event
SUBSCRIBE consumer3 system_log

# Unsubscribe from event
UNSUBSCRIBE consumer3 user_message
```

## Benefits

1. **Decoupling**: Producers don't need to know about consumers
2. **Scalability**: Add/remove consumers without restarting server
3. **Flexibility**: Consumers subscribe only to events they care about
4. **Event-driven**: Natural fit for event-driven architectures
5. **Protocol-agnostic**: Supports multiple protocols (TCP now, extensible)

## Future Enhancements

- Support for wildcard event subscriptions (e.g., "user.*")
- Event filtering/transformation
- Message persistence for offline consumers
- WebSocket protocol support
- REST API for registration
