use std::io::{Result, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::transport::client_interface::Client;

/// TCP client for sending messages to external consumers
pub struct TcpClient {
    address: String,
    timeout: Duration,
}

impl TcpClient {
    /// Create a new TCP client
    pub fn new(address: String) -> Self {
        Self {
            address,
            timeout: Duration::from_secs(5),
        }
    }

    /// Create a new TCP client with custom timeout
    pub fn with_timeout(address: String, timeout: Duration) -> Self {
        Self { address, timeout }
    }
}

impl Client for TcpClient {
    fn send(&self, message: &str) -> Result<()> {
        // Parse address and connect to the external consumer
        let addr = self.address.to_socket_addrs()
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid address format: {}", self.address)
            ))?
            .next()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Could not resolve address: {}", self.address)
            ))?;
        
        let mut stream = TcpStream::connect_timeout(&addr, self.timeout)?;
        
        // Set write timeout
        stream.set_write_timeout(Some(self.timeout))?;
        
        // Send message with newline
        let message_with_newline = if message.ends_with('\n') {
            message.to_string()
        } else {
            format!("{}\n", message)
        };
        
        stream.write_all(message_with_newline.as_bytes())?;
        stream.flush()?;
        
        Ok(())
    }

    fn protocol_name(&self) -> &str {
        "TCP"
    }

    fn address(&self) -> &str {
        &self.address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_client_creation() {
        let client = TcpClient::new("127.0.0.1:9000".to_string());
        assert_eq!(client.protocol_name(), "TCP");
        assert_eq!(client.address(), "127.0.0.1:9000");
    }

    #[test]
    fn test_tcp_client_with_timeout() {
        let client = TcpClient::with_timeout(
            "127.0.0.1:9000".to_string(),
            Duration::from_secs(10),
        );
        assert_eq!(client.protocol_name(), "TCP");
        assert_eq!(client.address(), "127.0.0.1:9000");
    }

    #[test]
    fn test_tcp_client_invalid_address() {
        let client = TcpClient::new("invalid-address".to_string());
        // Should fail when trying to send
        let result = client.send("test");
        assert!(result.is_err());
    }
}
