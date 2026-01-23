use crate::transport;
use std::env;

pub fn boot() {
    // Allow overriding the default address via environment variable
    let address = env::var("TRANSPORT_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:49152".to_string());
    
    println!("Initializing transport server...");
    println!("Address: {} (override with TRANSPORT_ADDRESS env var)", address);
    
    // Example: Add producers to forward messages to external consumers
    // You can configure these via environment variables or configuration
    setup_producers();
    
    match transport::bind_transport_with_address(address) {
        Ok(_) => {
            println!("Transport server started successfully!");
        }
        Err(e) => {
            eprintln!("Failed to bind transport: {}", e);
            eprintln!("\nTroubleshooting tips:");
            eprintln!("  • Try a different port: set TRANSPORT_ADDRESS=0.0.0.0:49153");
            eprintln!("  • Check if port is in use: netstat -ano | findstr :49152");
            eprintln!("  • Run as administrator if needed");
            eprintln!("  • Check Windows Firewall rules");
            eprintln!("  • Add exception in Windows Defender/Antivirus");
            std::process::exit(1);
        }
    }
}

/// Setup producers for forwarding messages to external consumers
fn setup_producers() {
    // Example: Add producers from environment variables
    // Format: PRODUCER_<ID>=<protocol>://<address>
    // Example: PRODUCER_CONSUMER1=tcp://127.0.0.1:9000
    
    // Check for producer configurations
    let mut producer_count = 0;
    
    // Example producer 1 (if configured)
    if let Ok(addr) = env::var("PRODUCER_CONSUMER1") {
        if let Err(e) = transport::add_tcp_producer("consumer-1".to_string(), addr.clone()) {
            eprintln!("Warning: Failed to add producer consumer-1: {}", e);
        } else {
            println!("✓ Added producer 'consumer-1' -> {}", addr);
            producer_count += 1;
        }
    }
    
    // Example producer 2 (if configured)
    if let Ok(addr) = env::var("PRODUCER_CONSUMER2") {
        if let Err(e) = transport::add_tcp_producer("consumer-2".to_string(), addr.clone()) {
            eprintln!("Warning: Failed to add producer consumer-2: {}", e);
        } else {
            println!("✓ Added producer 'consumer-2' -> {}", addr);
            producer_count += 1;
        }
    }
    
    if producer_count > 0 {
        println!("Configured {} producer(s) for message forwarding", producer_count);
    } else {
        println!("No producers configured. Messages will only be processed locally.");
        println!("To add producers, set environment variables:");
        println!("  PRODUCER_CONSUMER1=tcp://127.0.0.1:9000");
        println!("  PRODUCER_CONSUMER2=tcp://127.0.0.1:9001");
    }
}