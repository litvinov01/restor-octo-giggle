use std::io::{BufRead, BufReader, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::transport::transport_interface::{MessageConsumer, TransportProtocol};

/// Format error message with Windows-specific suggestions
fn format_error(e: &std::io::Error) -> String {
    match e.kind() {
        ErrorKind::PermissionDenied => {
            format!("{} - Try running as administrator or use a different port", e)
        }
        ErrorKind::AddrInUse => {
            format!("{} - Port is already in use. Try a different port", e)
        }
        ErrorKind::AddrNotAvailable => {
            format!("{} - Address not available. Try binding to 0.0.0.0 instead of 127.0.0.1", e)
        }
        _ => format!("{}", e)
    }
}

/// TCP transport protocol implementation
pub struct TcpTransport;

impl TcpTransport {
    /// Create a new TCP transport instance
    pub fn new(_address: String) -> Self {
        Self
    }
}

impl TransportProtocol for TcpTransport {
    fn listen(&self, address: &str, consumer: MessageConsumer) -> Result<()> {
        // Try to bind with better error handling
        let listener = match TcpListener::bind(address) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind TCP listener to {}: {}", address, e);
                eprintln!("Error details: {}", format_error(&e));
                eprintln!("Suggestions:");
                eprintln!("  1. Check if the port is already in use");
                eprintln!("  2. Try a different port number (e.g., 49153, 49154)");
                eprintln!("  3. Run as administrator if needed");
                eprintln!("  4. Check Windows Firewall settings");
                eprintln!("  5. Check antivirus/Windows Defender exclusions");
                return Err(e);
            }
        };
        
        let consumer = Arc::new(consumer);

        println!("TCP Transport successfully listening on {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let consumer = Arc::clone(&consumer);
                    let addr = stream.peer_addr()?;
                    
                    thread::spawn(move || {
                        if let Err(e) = handle_client(stream, consumer) {
                            eprintln!("Error handling client {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }

    fn protocol_name(&self) -> &str {
        "TCP"
    }
}

/// Handle individual client connection
fn handle_client(stream: TcpStream, consumer: Arc<MessageConsumer>) -> Result<()> {
    let reader = BufReader::new(&stream);
    
    for line in reader.lines() {
        let message = line?;
        
        if message.is_empty() {
            continue;
        }
        
        // Consume the message
        consumer(message);
    }
    
    Ok(())
}
