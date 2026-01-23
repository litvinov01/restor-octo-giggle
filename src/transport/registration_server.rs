use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::transport::{
    get_producer_pool,
    ClientFactory,
    message::EventMessage,
};

/// Default registration server port
const DEFAULT_REGISTRATION_PORT: u16 = 49153;

/// Start the registration server for external consumer registration
pub fn start_registration_server(port: u16) -> Result<()> {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)?;
    
    println!("Registration server listening on {}", address);
    
    let pool = get_producer_pool();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = Arc::clone(&pool);
                thread::spawn(move || {
                    if let Err(e) = handle_registration(stream, pool) {
                        eprintln!("Registration error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Registration connection failed: {}", e);
            }
        }
    }
    
    Ok(())
}

/// Handle consumer registration connection
fn handle_registration(mut stream: TcpStream, pool: Arc<crate::transport::ProducerPool>) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    
    println!("[Registration] New connection from {}", peer_addr);
    
    // Send welcome message
    writeln!(stream, "REGISTRATION_SERVER:1.0")?;
    writeln!(stream, "Commands: REGISTER <id> <protocol>://<address> [events...]")?;
    writeln!(stream, "          SUBSCRIBE <id> <event_name>")?;
    writeln!(stream, "          UNSUBSCRIBE <id> <event_name>")?;
    writeln!(stream, "          LIST")?;
    writeln!(stream, "          QUIT")?;
    stream.flush()?;
    
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        
        if bytes_read == 0 {
            break; // Connection closed
        }
        
        let command = line.trim();
        if command.is_empty() {
            continue;
        }
        
        let response = process_registration_command(command, &pool);
        
        match response {
            Ok(msg) => {
                writeln!(stream, "OK:{}", msg)?;
            }
            Err(e) => {
                writeln!(stream, "ERROR:{}", e)?;
            }
        }
        
        stream.flush()?;
        
        if command.eq_ignore_ascii_case("QUIT") {
            break;
        }
    }
    
    println!("[Registration] Connection closed: {}", peer_addr);
    Ok(())
}

/// Process registration command
fn process_registration_command(
    command: &str,
    pool: &Arc<crate::transport::ProducerPool>,
) -> Result<String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Empty command",
        ));
    }
    
    match parts[0].to_uppercase().as_str() {
        "REGISTER" => {
            if parts.len() < 3 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Usage: REGISTER <id> <protocol>://<address> [events...]",
                ));
            }
            
            let id = parts[1].to_string();
            let uri = parts[2];
            
            // Parse URI: protocol://address
            if !uri.contains("://") {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid URI format. Expected: protocol://address",
                ));
            }
            
            let (protocol, address) = uri.split_once("://")
                .ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid URI format",
                ))?;
            
            // Get event subscriptions (optional)
            let events: Vec<&str> = parts.iter().skip(3).copied().collect();
            
            // Create client based on protocol
            let client = match protocol.to_uppercase().as_str() {
                "TCP" => {
                    ClientFactory::create_tcp_client(address.to_string())
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Unsupported protocol: {}", protocol),
                    ));
                }
            };
            
            // Add producer with events
            if events.is_empty() {
                pool.add_producer(id.clone(), client)?;
                Ok(format!("Producer '{}' registered", id))
            } else {
                pool.add_producer_with_events(id.clone(), client, &events)?;
                Ok(format!("Producer '{}' registered with events: {:?}", id, events))
            }
        }
        
        "SUBSCRIBE" => {
            if parts.len() != 3 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Usage: SUBSCRIBE <id> <event_name>",
                ));
            }
            
            let id = parts[1];
            let event = parts[2];
            
            pool.subscribe_producer_to_event(id, event)?;
            Ok(format!("Producer '{}' subscribed to event '{}'", id, event))
        }
        
        "UNSUBSCRIBE" => {
            if parts.len() != 3 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Usage: UNSUBSCRIBE <id> <event_name>",
                ));
            }
            
            let id = parts[1];
            let event = parts[2];
            
            pool.unsubscribe_producer_from_event(id, event)?;
            Ok(format!("Producer '{}' unsubscribed from event '{}'", id, event))
        }
        
        "LIST" => {
            let producer_ids = pool.get_producer_ids();
            let events = pool.get_subscribed_events();
            
            let mut info = format!("Producers: {}\n", producer_ids.len());
            for id in &producer_ids {
                if let Some(producer) = pool.get_producer(id) {
                    let subscribed = producer.subscribed_events();
                    info.push_str(&format!("  {} -> {} (events: {:?})\n", 
                        id, producer.address(), subscribed));
                }
            }
            info.push_str(&format!("Events: {:?}", events));
            
            Ok(info)
        }
        
        "QUIT" => {
            Ok("Goodbye".to_string())
        }
        
        _ => {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown command: {}", parts[0]),
            ))
        }
    }
}

/// Start registration server on default port
pub fn start_registration_server_default() -> Result<()> {
    start_registration_server(DEFAULT_REGISTRATION_PORT)
}
