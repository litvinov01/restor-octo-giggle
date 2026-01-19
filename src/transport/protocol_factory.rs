use std::io::Result;
use std::sync::Arc;

use crate::transport::protocols::TcpTransport;
use crate::transport::server_config::ProtocolType;
use crate::transport::transport_interface::TransportProtocol;

/// Factory for creating transport protocol instances based on configuration
pub struct ProtocolFactory;

impl ProtocolFactory {
    /// Create a transport protocol instance based on the protocol type
    pub fn create(protocol_type: ProtocolType, address: &str) -> Result<Arc<dyn TransportProtocol>> {
        match protocol_type {
            ProtocolType::Tcp => {
                Ok(Arc::new(TcpTransport::new(address.to_string())))
            }
            // Future protocols can be added here:
            // ProtocolType::Udp => {
            //     Ok(Arc::new(UdpTransport::new(address.to_string())))
            // }
        }
    }

    /// Get protocol name by type
    pub fn protocol_name(protocol_type: ProtocolType) -> &'static str {
        protocol_type.as_str()
    }
}
