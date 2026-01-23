use std::sync::{Arc, OnceLock};

use super::producer_pool::ProducerPool;

/// Global producer pool registry
static PRODUCER_POOL: OnceLock<Arc<ProducerPool>> = OnceLock::new();

/// Initialize the global producer pool
pub fn init_producer_pool() -> Arc<ProducerPool> {
    PRODUCER_POOL.get_or_init(|| Arc::new(ProducerPool::new())).clone()
}

/// Get the global producer pool
pub fn get_producer_pool() -> Arc<ProducerPool> {
    PRODUCER_POOL.get_or_init(|| Arc::new(ProducerPool::new())).clone()
}
