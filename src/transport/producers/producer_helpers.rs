use std::io::Result;
use std::sync::Arc;

use super::{ProducerPool, ClientFactory, get_producer_pool};

/// Helper functions for managing producers in the pool

/// Add a TCP producer to the global producer pool
pub fn add_tcp_producer(id: String, address: String) -> Result<()> {
    let pool = get_producer_pool();
    let client = ClientFactory::create_tcp_client(address);
    pool.add_producer(id, client)
}

/// Add a TCP producer with custom timeout
pub fn add_tcp_producer_with_timeout(
    id: String, 
    address: String, 
    timeout: std::time::Duration
) -> Result<()> {
    let pool = get_producer_pool();
    let client = ClientFactory::create_tcp_client_with_timeout(address, timeout);
    pool.add_producer(id, client)
}

/// Forward a message to a specific producer by ID
pub fn forward_to_producer(producer_id: &str, message: &str) -> Result<()> {
    let pool = get_producer_pool();
    pool.forward_to(producer_id, message)
}

/// Forward a message to all producers
pub fn forward_to_all_producers(message: &str) -> Vec<(String, Result<()>)> {
    let pool = get_producer_pool();
    pool.forward_to_all(message)
}
