use std::io::Result;

use crate::transport::{Server, ServerConfig, MessageConsumer};

/// Bind and start the transport server with default settings (TCP)
pub fn bind_transport() -> Result<()> {
    bind_transport_with_config(ServerConfig::new())
}

/// Bind and start the transport server with a custom address (defaults to TCP)
pub fn bind_transport_with_address(address: String) -> Result<()> {
    let config = ServerConfig::with_address(address);
    bind_transport_with_config(config)
}

/// Bind and start the transport server with custom configuration
pub fn bind_transport_with_config(config: ServerConfig) -> Result<()> {
    // Create server with configuration
    let mut server = Server::with_config(config);
    
    // Define message consumer
    let consumer: MessageConsumer = Box::new(|message| {
        println!("[Message Consumer] Received: {}", message);
        // Add your message processing logic here
        process_message(message);
    });
    
    // Initialize and start the server
    server.initialize_and_start(consumer)?;
    
    println!("Transport server initialized with {} protocol on {}", 
             server.config().protocol.as_str(),
             server.config().address);
    
    Ok(())
}

/// Process received messages
fn process_message(message: String) {
    // Example message processing
    println!("Processing message: {}", message);
    
    // Add your message handling logic here
    // For example:
    // - Parse commands
    // - Route to handlers
    // - Send acknowledgments
    // etc.
}
