use std::io::Result;
use std::sync::Arc;
use std::thread;

use crate::transport::{
    Server, 
    ServerConfig, 
    MessageConsumer, 
    get_producer_pool,
    EventMessage,
    start_registration_server_default,
};

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
    // Initialize global producer pool
    let producer_pool = get_producer_pool();
    
    // Start registration server in background thread
    thread::spawn(move || {
        if let Err(e) = start_registration_server_default() {
            eprintln!("Registration server error: {}", e);
        }
    });
    
    // Create server with configuration
    let mut server = Server::with_config(config);
    
    // Clone producer pool for the consumer closure
    let pool = Arc::clone(&producer_pool);
    
    // Define message consumer that routes by event_name
    let consumer: MessageConsumer = Box::new(move |message| {
        println!("[Message Consumer] Received: {}", message);
        
        // Try to parse as EventMessage (JSON or simple format)
        let event_message = match EventMessage::from_json(&message) {
            Ok(msg) => msg,
            Err(_) => {
                // Try simple format: "event_name:message"
                EventMessage::from_simple_format(&message)
            }
        };
        
        println!("[Event Router] Event: '{}', Message: '{}'", 
                 event_message.event_name, event_message.msg);
        
        // Forward message to producers subscribed to this event
        let results = pool.forward_to_event(&event_message.event_name, &event_message.msg);
        
        // Log forwarding results
        for (producer_id, result) in results {
            match result {
                Ok(_) => println!("[Producer {}] Event '{}' forwarded successfully", 
                                 producer_id, event_message.event_name),
                Err(e) => eprintln!("[Producer {}] Failed to forward event '{}': {}", 
                                   producer_id, event_message.event_name, e),
            }
        }
        
        // Also process locally
        process_message(message);
    });
    
    // Initialize and start the server
    server.initialize_and_start(consumer)?;
    
    println!("Transport server initialized with {} protocol on {}", 
             server.config().protocol.as_str(),
             server.config().address);
    println!("Registration server started on port 49153");
    
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
