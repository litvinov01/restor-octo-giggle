use std::io::Result;

/// Trait for clients that can send messages to external consumers
pub trait Client: Send + Sync {
    /// Send a message to the external consumer
    fn send(&self, message: &str) -> Result<()>;
    
    /// Get the client protocol name
    fn protocol_name(&self) -> &str;
    
    /// Get the target address
    fn address(&self) -> &str;
}
