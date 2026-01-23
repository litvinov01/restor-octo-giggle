# Producer Pool System

The producer pool system allows forwarding messages received by the transport server to external consumers using injected clients with different protocol support.

## Architecture

```
Message Received
    ↓
MessageConsumer (in bind_transport)
    ↓
ProducerPool.forward_to_all()
    ↓
Producer.forward() (for each producer)
    ↓
Client.send() (injected client - TCP, UDP, etc.)
    ↓
External Consumer
```

## Components

### 1. Client Trait (`client_interface.rs`)

Defines the interface for clients that can send messages:

```rust
pub trait Client: Send + Sync {
    fn send(&self, message: &str) -> Result<()>;
    fn protocol_name(&self) -> &str;
    fn address(&self) -> &str;
}
```

### 2. TCP Client (`clients/tcp.rs`)

Implementation of `Client` trait for TCP protocol:

```rust
let client = TcpClient::new("127.0.0.1:9000".to_string());
client.send("Hello, external consumer!")?;
```

### 3. Producer (`producer.rs`)

Producer with injected client:

```rust
let producer = Producer::new("consumer-1".to_string(), client);
producer.forward("Message to forward")?;
```

### 4. Producer Pool (`producer_pool.rs`)

Manages multiple producers:

```rust
let pool = ProducerPool::new();
pool.add_producer("consumer-1".to_string(), client)?;
pool.forward_to_all("Broadcast message")?;
```

### 5. Client Factory (`producer_factory.rs`)

Factory for creating clients:

```rust
let tcp_client = ClientFactory::create_tcp_client("127.0.0.1:9000".to_string());
let tcp_client_with_timeout = ClientFactory::create_tcp_client_with_timeout(
    "127.0.0.1:9001".to_string(),
    Duration::from_secs(10),
);
```

## Usage Examples

### Basic Usage

```rust
use crate::transport::{add_tcp_producer, get_producer_pool};

// Add a producer
add_tcp_producer("consumer-1".to_string(), "127.0.0.1:9000".to_string())?;

// Forward message to all producers (automatically done by message consumer)
// Or manually forward:
let pool = get_producer_pool();
pool.forward_to("consumer-1", "Hello!")?;
```

### Advanced Usage

```rust
use crate::transport::{ProducerPool, ClientFactory, TcpClient};
use std::sync::Arc;
use std::time::Duration;

// Create pool
let pool = ProducerPool::new();

// Add multiple producers
let client1 = ClientFactory::create_tcp_client("127.0.0.1:9000".to_string());
pool.add_producer("consumer-1".to_string(), client1)?;

let client2 = ClientFactory::create_tcp_client_with_timeout(
    "127.0.0.1:9001".to_string(),
    Duration::from_secs(10),
);
pool.add_producer("consumer-2".to_string(), client2)?;

// Forward to specific producer
pool.forward_to("consumer-1", "Message for consumer 1")?;

// Forward to all producers
let results = pool.forward_to_all("Broadcast message");
for (id, result) in results {
    match result {
        Ok(_) => println!("✓ Forwarded to {}", id),
        Err(e) => eprintln!("✗ Failed to forward to {}: {}", id, e),
    }
}

// Forward to multiple specific producers
let results = pool.forward_to_many(&["consumer-1", "consumer-2"], "Multi-target");
```

### Using Helper Functions

```rust
use crate::transport::{
    add_tcp_producer,
    add_tcp_producer_with_timeout,
    forward_to_producer,
    forward_to_all_producers,
};

// Add producers
add_tcp_producer("consumer-1".to_string(), "127.0.0.1:9000".to_string())?;
add_tcp_producer_with_timeout(
    "consumer-2".to_string(),
    "127.0.0.1:9001".to_string(),
    Duration::from_secs(10),
)?;

// Forward messages
forward_to_producer("consumer-1", "Hello!")?;
let results = forward_to_all_producers("Broadcast");
```

### Environment Variable Configuration

You can configure producers via environment variables in `boot/mod.rs`:

```bash
# Set producer addresses
export PRODUCER_CONSUMER1=tcp://127.0.0.1:9000
export PRODUCER_CONSUMER2=tcp://127.0.0.1:9001
```

## Integration with Message Consumer

The producer pool is automatically integrated with the message consumer in `bind_transport_with_config()`:

```rust
let consumer: MessageConsumer = Box::new(move |message| {
    // Forward to all producers
    let results = pool.forward_to_all(&message);
    
    // Log results
    for (producer_id, result) in results {
        match result {
            Ok(_) => println!("[Producer {}] Forwarded", producer_id),
            Err(e) => eprintln!("[Producer {}] Failed: {}", producer_id, e),
        }
    }
    
    // Also process locally
    process_message(message);
});
```

## Adding New Protocol Clients

To add a new protocol (e.g., UDP):

1. **Implement the Client trait**:
   ```rust
   // clients/udp.rs
   impl Client for UdpClient {
       fn send(&self, message: &str) -> Result<()> {
           // UDP implementation
       }
       // ...
   }
   ```

2. **Add to clients module**:
   ```rust
   // clients/mod.rs
   pub mod udp;
   pub use udp::UdpClient;
   ```

3. **Add factory method**:
   ```rust
   // producer_factory.rs
   impl ClientFactory {
       pub fn create_udp_client(address: String) -> Arc<dyn Client> {
           Arc::new(UdpClient::new(address))
       }
   }
   ```

4. **Use it**:
   ```rust
   let udp_client = ClientFactory::create_udp_client("127.0.0.1:9000".to_string());
   pool.add_producer("udp-consumer".to_string(), udp_client)?;
   ```

## Thread Safety

- `ProducerPool` uses `Arc<Mutex<HashMap>>` for thread-safe access
- All clients must implement `Send + Sync`
- Producers can be safely accessed from multiple threads

## Error Handling

The producer pool handles errors gracefully:

- Individual producer failures don't stop others
- Results are returned as `Vec<(String, Result<()>)>` for inspection
- Failed forwards are logged but don't crash the system

## Best Practices

1. **Use meaningful producer IDs**: e.g., "database-writer", "log-aggregator"
2. **Handle errors**: Check results when forwarding to multiple producers
3. **Configure timeouts**: Use `create_tcp_client_with_timeout` for long-running connections
4. **Monitor producer health**: Periodically check if producers are reachable
5. **Use environment variables**: Configure producers via env vars for flexibility
