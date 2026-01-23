//! Test consumer server for integration testing with JS driver
//! 
//! This binary starts a simple consumer server that:
//! - Listens for TCP connections
//! - Receives messages from JS driver (producer)
//! - Logs all received messages
//! - Can be used for integration testing

use std::io::{stdin, BufRead};
mod boot;
mod transport;

fn main() {
    println!("==========================================");
    println!("Test Consumer Server");
    println!("==========================================");
    println!("\nStarting test consumer server...");
    println!("This server will receive messages from JS driver (producer)");
    println!("\nServer Configuration:");
    
    // Initialize transport server
    boot::boot();
    
    println!("\n==========================================");
    println!("Consumer server is running!");
    println!("==========================================");
    println!("\nServer is ready to receive messages from:");
    println!("  - JS Driver: node drivers/js/test-consumer-producer.js");
    println!("\nPress 'q' and Enter to quit, or Ctrl+C to exit.\n");
    
    // Keep the main thread alive and allow user input
    let stdin = stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.eq_ignore_ascii_case("q") || trimmed.eq_ignore_ascii_case("quit") {
                    println!("\nShutting down consumer server...");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    
    println!("Consumer server stopped.");
}
