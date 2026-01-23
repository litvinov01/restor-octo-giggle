use std::io::Result;
use std::sync::Arc;

use crate::transport::client_interface::Client;
use crate::transport::clients::TcpClient;

/// Factory for creating clients based on protocol type
pub struct ClientFactory;

impl ClientFactory {
    /// Create a client instance based on protocol type
    pub fn create_tcp_client(address: String) -> Arc<dyn Client> {
        Arc::new(TcpClient::new(address))
    }

    /// Create a client with custom timeout
    pub fn create_tcp_client_with_timeout(address: String, timeout: std::time::Duration) -> Arc<dyn Client> {
        Arc::new(TcpClient::with_timeout(address, timeout))
    }

    // Future: Add other protocol clients here
    // pub fn create_udp_client(address: String) -> Arc<dyn Client> { ... }
}
