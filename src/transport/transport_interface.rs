use std::io::Result;

/// Trait for message consumer callbacks
pub type MessageConsumer = Box<dyn Fn(String) + Send + Sync>;

/// Trait defining transport protocol behavior
pub trait TransportProtocol: Send + Sync {
    /// Start listening on the specified address
    fn listen(&self, address: &str, consumer: MessageConsumer) -> Result<()>;
    
    /// Get the protocol name
    fn protocol_name(&self) -> &str;
}
