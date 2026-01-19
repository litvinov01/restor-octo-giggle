use std::io::Result;
use std::sync::Arc;

use crate::transport::transport_interface::{MessageConsumer, TransportProtocol};

/// Transport server that manages protocol initialization and message consumption
pub struct TransportServer {
    pub(crate) protocol: Arc<dyn TransportProtocol>,
    address: String,
}

impl TransportServer {
    /// Create a new transport server with the specified protocol
    pub fn new(protocol: Arc<dyn TransportProtocol>, address: String) -> Self {
        Self { protocol, address }
    }

    /// Start the transport server and begin consuming messages
    pub fn start(&self, consumer: MessageConsumer) -> Result<()> {
        println!("Starting {} transport server on {}", 
                 self.protocol.protocol_name(), 
                 self.address);
        
        self.protocol.listen(&self.address, consumer)
    }

    /// Get the server address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get the protocol name
    pub fn protocol_name(&self) -> &str {
        self.protocol.protocol_name()
    }
}
