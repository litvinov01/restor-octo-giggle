use std::collections::{HashMap, HashSet};
use std::io::Result;
use std::sync::{Arc, Mutex};

use crate::transport::client_interface::Client;
use super::producer::Producer;
use super::observer::Observer;

/// Pool of producers for forwarding messages to external consumers
/// Supports event-based routing using observer pattern
pub struct ProducerPool {
    producers: Arc<Mutex<HashMap<String, Arc<Producer>>>>,
    // Event -> Set of producer IDs subscribed to this event
    event_subscriptions: Arc<Mutex<HashMap<String, HashSet<String>>>>,
}

impl ProducerPool {
    /// Create a new producer pool
    pub fn new() -> Self {
        Self {
            producers: Arc::new(Mutex::new(HashMap::new())),
            event_subscriptions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a producer to the pool
    pub fn add_producer(&self, id: String, client: Arc<dyn Client>) -> Result<()> {
        let producer = Arc::new(Producer::new(id.clone(), client));
        let mut producers = self.producers.lock().unwrap();
        producers.insert(id.clone(), producer);
        Ok(())
    }

    /// Add a producer with initial event subscriptions
    pub fn add_producer_with_events(
        &self,
        id: String,
        client: Arc<dyn Client>,
        events: &[&str],
    ) -> Result<()> {
        let producer = Arc::new(Producer::new(id.clone(), client));
        
        // Subscribe to events
        for event in events {
            producer.subscribe(event);
        }
        
        // Update event subscriptions index
        {
            let mut producers = self.producers.lock().unwrap();
            producers.insert(id.clone(), producer);
        }
        
        // Update event -> producer mapping
        {
            let mut event_subs = self.event_subscriptions.lock().unwrap();
            for event in events {
                event_subs
                    .entry(event.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(id.clone());
            }
        }
        
        Ok(())
    }

    /// Remove a producer from the pool
    pub fn remove_producer(&self, id: &str) -> Option<Arc<Producer>> {
        let producer = {
            let mut producers = self.producers.lock().unwrap();
            producers.remove(id)
        };
        
        // Remove from event subscriptions
        if producer.is_some() {
            let mut event_subs = self.event_subscriptions.lock().unwrap();
            for producer_ids in event_subs.values_mut() {
                producer_ids.remove(id);
            }
            // Clean up empty event entries
            event_subs.retain(|_, producer_ids| !producer_ids.is_empty());
        }
        
        producer
    }

    /// Get a producer by ID
    pub fn get_producer(&self, id: &str) -> Option<Arc<Producer>> {
        let producers = self.producers.lock().unwrap();
        producers.get(id).map(|p| Arc::clone(p))
    }

    /// Forward a message to a specific producer
    pub fn forward_to(&self, producer_id: &str, message: &str) -> Result<()> {
        let producer = self.get_producer(producer_id)
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Producer not found: {}", producer_id),
            ))?;
        
        producer.forward(message)
    }

    /// Forward a message to all producers in the pool
    pub fn forward_to_all(&self, message: &str) -> Vec<(String, Result<()>)> {
        let producers = {
            let producers = self.producers.lock().unwrap();
            producers.iter()
                .map(|(id, producer)| (id.clone(), Arc::clone(producer)))
                .collect::<Vec<_>>()
        };

        producers.into_iter()
            .map(|(id, producer)| {
                let result = producer.forward(message);
                (id, result)
            })
            .collect()
    }

    /// Forward a message to multiple specific producers
    pub fn forward_to_many(&self, producer_ids: &[&str], message: &str) -> Vec<(String, Result<()>)> {
        producer_ids.iter()
            .map(|&id| {
                let result = self.forward_to(id, message);
                (id.to_string(), result)
            })
            .collect()
    }

    /// Get all producer IDs
    pub fn get_producer_ids(&self) -> Vec<String> {
        let producers = self.producers.lock().unwrap();
        producers.keys().cloned().collect()
    }

    /// Get the number of producers in the pool
    pub fn count(&self) -> usize {
        let producers = self.producers.lock().unwrap();
        producers.len()
    }

    /// Check if a producer exists
    pub fn has_producer(&self, id: &str) -> bool {
        let producers = self.producers.lock().unwrap();
        producers.contains_key(id)
    }

    /// Subscribe a producer to an event
    pub fn subscribe_producer_to_event(&self, producer_id: &str, event_name: &str) -> Result<()> {
        let producer = self.get_producer(producer_id)
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Producer not found: {}", producer_id),
            ))?;
        
        producer.subscribe(event_name);
        
        // Update event subscriptions index
        let mut event_subs = self.event_subscriptions.lock().unwrap();
        event_subs
            .entry(event_name.to_string())
            .or_insert_with(HashSet::new)
            .insert(producer_id.to_string());
        
        Ok(())
    }

    /// Unsubscribe a producer from an event
    pub fn unsubscribe_producer_from_event(&self, producer_id: &str, event_name: &str) -> Result<()> {
        let producer = self.get_producer(producer_id)
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Producer not found: {}", producer_id),
            ))?;
        
        producer.unsubscribe(event_name);
        
        // Update event subscriptions index
        let mut event_subs = self.event_subscriptions.lock().unwrap();
        if let Some(producer_ids) = event_subs.get_mut(event_name) {
            producer_ids.remove(producer_id);
            if producer_ids.is_empty() {
                event_subs.remove(event_name);
            }
        }
        
        Ok(())
    }

    /// Forward message to producers subscribed to a specific event
    pub fn forward_to_event(&self, event_name: &str, message: &str) -> Vec<(String, Result<()>)> {
        let producer_ids = {
            let event_subs = self.event_subscriptions.lock().unwrap();
            event_subs
                .get(event_name)
                .map(|ids| ids.iter().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        };

        producer_ids.into_iter()
            .filter_map(|id| {
                self.get_producer(&id).map(|producer| {
                    let result = producer.forward(message);
                    (id, result)
                })
            })
            .collect()
    }

    /// Get all events that have subscribers
    pub fn get_subscribed_events(&self) -> Vec<String> {
        let event_subs = self.event_subscriptions.lock().unwrap();
        event_subs.keys().cloned().collect()
    }

    /// Get producers subscribed to an event
    pub fn get_event_subscribers(&self, event_name: &str) -> Vec<String> {
        let event_subs = self.event_subscriptions.lock().unwrap();
        event_subs
            .get(event_name)
            .map(|ids| ids.iter().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for ProducerPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::clients::TcpClient;

    struct MockClient {
        sent_messages: Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl Client for MockClient {
        fn send(&self, message: &str) -> Result<()> {
            self.sent_messages.lock().unwrap().push(message.to_string());
            Ok(())
        }

        fn protocol_name(&self) -> &str {
            "MOCK"
        }

        fn address(&self) -> &str {
            "mock://test"
        }
    }

    #[test]
    fn test_producer_pool_creation() {
        let pool = ProducerPool::new();
        assert_eq!(pool.count(), 0);
    }

    #[test]
    fn test_add_producer() {
        let pool = ProducerPool::new();
        let client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        
        pool.add_producer("producer-1".to_string(), client).unwrap();
        assert_eq!(pool.count(), 1);
        assert!(pool.has_producer("producer-1"));
    }

    #[test]
    fn test_remove_producer() {
        let pool = ProducerPool::new();
        let client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        
        pool.add_producer("producer-1".to_string(), client).unwrap();
        assert_eq!(pool.count(), 1);
        
        pool.remove_producer("producer-1");
        assert_eq!(pool.count(), 0);
        assert!(!pool.has_producer("producer-1"));
    }

    #[test]
    fn test_forward_to_producer() {
        let pool = ProducerPool::new();
        let sent = Arc::new(std::sync::Mutex::new(Vec::new()));
        let client = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent),
        });
        
        pool.add_producer("producer-1".to_string(), client).unwrap();
        pool.forward_to("producer-1", "test message").unwrap();
        
        let messages = sent.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "test message");
    }

    #[test]
    fn test_forward_to_all() {
        let pool = ProducerPool::new();
        let sent1 = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sent2 = Arc::new(std::sync::Mutex::new(Vec::new()));
        
        let client1 = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent1),
        });
        let client2 = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent2),
        });
        
        pool.add_producer("producer-1".to_string(), client1).unwrap();
        pool.add_producer("producer-2".to_string(), client2).unwrap();
        
        let results = pool.forward_to_all("broadcast message");
        assert_eq!(results.len(), 2);
        
        // Check all succeeded
        for (_, result) in results {
            assert!(result.is_ok());
        }
        
        assert_eq!(sent1.lock().unwrap().len(), 1);
        assert_eq!(sent2.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_add_producer_with_events() {
        let pool = ProducerPool::new();
        let client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        
        pool.add_producer_with_events(
            "producer-1".to_string(),
            client,
            &["event1", "event2"],
        ).unwrap();
        
        let events = pool.get_subscribed_events();
        assert!(events.contains(&"event1".to_string()));
        assert!(events.contains(&"event2".to_string()));
        
        let subscribers = pool.get_event_subscribers("event1");
        assert_eq!(subscribers.len(), 1);
        assert_eq!(subscribers[0], "producer-1");
    }

    #[test]
    fn test_forward_to_event() {
        let pool = ProducerPool::new();
        let sent1 = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sent2 = Arc::new(std::sync::Mutex::new(Vec::new()));
        
        let client1 = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent1),
        });
        let client2 = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent2),
        });
        
        pool.add_producer_with_events("producer-1".to_string(), client1, &["event1"]).unwrap();
        pool.add_producer_with_events("producer-2".to_string(), client2, &["event2"]).unwrap();
        
        let results = pool.forward_to_event("event1", "event message");
        assert_eq!(results.len(), 1);
        assert_eq!(sent1.lock().unwrap().len(), 1);
        assert_eq!(sent2.lock().unwrap().len(), 0); // Not subscribed to event1
    }

    #[test]
    fn test_subscribe_unsubscribe_event() {
        let pool = ProducerPool::new();
        let client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        
        pool.add_producer("producer-1".to_string(), client).unwrap();
        pool.subscribe_producer_to_event("producer-1", "event1").unwrap();
        
        let subscribers = pool.get_event_subscribers("event1");
        assert_eq!(subscribers.len(), 1);
        
        pool.unsubscribe_producer_from_event("producer-1", "event1").unwrap();
        let subscribers = pool.get_event_subscribers("event1");
        assert_eq!(subscribers.len(), 0);
    }

    #[test]
    fn test_get_producer_ids() {
        let pool = ProducerPool::new();
        let client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        
        pool.add_producer("producer-1".to_string(), client.clone()).unwrap();
        pool.add_producer("producer-2".to_string(), client).unwrap();
        
        let ids = pool.get_producer_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"producer-1".to_string()));
        assert!(ids.contains(&"producer-2".to_string()));
    }
}
