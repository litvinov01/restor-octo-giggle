/// Supported transport protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolType {
    /// TCP transport protocol
    Tcp,
    // Future protocols can be added here:
    // Udp,
    // WebSocket,
    // Http,
}

impl Default for ProtocolType {
    fn default() -> Self {
        ProtocolType::Tcp
    }
}

impl ProtocolType {
    /// Convert string to ProtocolType
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TCP" => Some(ProtocolType::Tcp),
            _ => None,
        }
    }

    /// Get protocol type as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ProtocolType::Tcp => "TCP",
        }
    }
}

/// Server configuration for initialization
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Transport protocol type to use
    pub protocol: ProtocolType,
    /// Server address to bind to
    pub address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            protocol: ProtocolType::default(),
            // Use 0.0.0.0 to bind to all interfaces and a higher port (49152+) 
            // to avoid Windows socket access restrictions and conflicts
            address: "0.0.0.0:49152".to_string(),
        }
    }
}

impl ServerConfig {
    /// Create a new server configuration with default settings (TCP)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new server configuration with custom address (defaults to TCP)
    pub fn with_address(address: String) -> Self {
        Self {
            protocol: ProtocolType::default(),
            address,
        }
    }

    /// Create a new server configuration with custom protocol and address
    pub fn with_protocol(protocol: ProtocolType, address: String) -> Self {
        Self { protocol, address }
    }

    /// Set the protocol type
    pub fn set_protocol(&mut self, protocol: ProtocolType) -> &mut Self {
        self.protocol = protocol;
        self
    }

    /// Set the server address
    pub fn set_address(&mut self, address: String) -> &mut Self {
        self.address = address;
        self
    }
}
