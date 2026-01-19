use crate::transport;
use std::env;

pub fn boot() {
    // Allow overriding the default address via environment variable
    let address = env::var("TRANSPORT_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:49152".to_string());
    
    println!("Initializing transport server...");
    println!("Address: {} (override with TRANSPORT_ADDRESS env var)", address);
    
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