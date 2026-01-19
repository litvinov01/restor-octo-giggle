use std::io::{stdin, BufRead};
mod boot;
mod transport;

fn main() {
    println!("Starting Rust Samples application...");
    
    // Initialize transport server
    boot::boot();
    
    println!("\nTransport server is running and ready to accept connections!");
    println!("Connect using: telnet 0.0.0.0 49152 (or nc/your TCP client)");
    println!("Press Ctrl+C to exit or type 'quit' to stop.\n");
    
    // Keep the main thread alive and allow user input
    let stdin = stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim().eq_ignore_ascii_case("quit") {
                    println!("Shutting down...");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
