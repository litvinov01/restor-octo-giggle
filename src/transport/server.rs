use std::io::Result;
use std::sync::Arc;
use std::thread;

use crate::transport::protocol_factory::ProtocolFactory;
use crate::transport::server_config::ServerConfig;
use crate::transport::transport_interface::MessageConsumer;
use crate::transport::transport_server::TransportServer;

/// Server that encapsulates transport server initialization and protocol mapping
pub struct Server {
    config: ServerConfig,
    transport_server: Option<TransportServer>,
}

impl Server {
    /// Create a new server with default configuration (TCP on 127.0.0.1:8080)
    pub fn new() -> Self {
        Self {
            config: ServerConfig::new(),
            transport_server: None,
        }
    }

    /// Create a new server with custom configuration
    pub fn with_config(config: ServerConfig) -> Self {
        Self {
            config,
            transport_server: None,
        }
    }

    /// Initialize the server with the configured protocol
    pub fn initialize(&mut self) -> Result<()> {
        // Create protocol instance based on configuration
        let protocol = ProtocolFactory::create(self.config.protocol, &self.config.address)?;
        
        // Create transport server with the protocol
        let transport_server = TransportServer::new(protocol, self.config.address.clone());
        
        self.transport_server = Some(transport_server);
        
        Ok(())
    }

    /// Start the server with a message consumer
    pub fn start(&self, consumer: MessageConsumer) -> Result<()> {
        let server = self.transport_server.as_ref()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::Other,
                "Server not initialized. Call initialize() first."
            ))?;

        // Start the server in a separate thread to avoid blocking
        let server_clone = TransportServer::new(
            Arc::clone(&server.protocol),
            server.address().to_string()
        );
        
        thread::spawn(move || {
            if let Err(e) = server_clone.start(consumer) {
                eprintln!("Transport server error: {}", e);
            }
        });

        Ok(())
    }

    /// Initialize and start the server in one call
    pub fn initialize_and_start(&mut self, consumer: MessageConsumer) -> Result<()> {
        self.initialize()?;
        self.start(consumer)
    }

    /// Get the current configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Update the configuration (requires re-initialization)
    pub fn set_config(&mut self, config: ServerConfig) {
        self.config = config;
        self.transport_server = None; // Invalidate existing server
    }

    /// Check if the server is initialized
    pub fn is_initialized(&self) -> bool {
        self.transport_server.is_some()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

