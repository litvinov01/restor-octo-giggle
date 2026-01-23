use std::io::Result;
use std::sync::Arc;

use crate::transport::client_interface::Client;
use super::observer::{Observer, EventSubscriptions};

/// Producer that forwards messages to external consumers using an injected client
/// Acts as an observer that can subscribe to events
pub struct Producer {
    client: Arc<dyn Client>,
    id: String,
    subscriptions: EventSubscriptions,
}

impl Producer {
    /// Create a new producer with an injected client
    pub fn new(id: String, client: Arc<dyn Client>) -> Self {
        Self {
            client,
            id,
            subscriptions: EventSubscriptions::new(),
        }
    }

    /// Forward a message to the external consumer
    pub fn forward(&self, message: &str) -> Result<()> {
        self.client.send(message)
    }

    /// Subscribe to an event
    pub fn subscribe(&self, event_name: &str) {
        self.subscriptions.subscribe(event_name);
    }

    /// Unsubscribe from an event
    pub fn unsubscribe(&self, event_name: &str) {
        self.subscriptions.unsubscribe(event_name);
    }

    /// Subscribe to multiple events
    pub fn subscribe_many(&self, event_names: &[&str]) {
        self.subscriptions.subscribe_many(event_names);
    }

    /// Check if subscribed to an event
    pub fn is_subscribed(&self, event_name: &str) -> bool {
        self.subscriptions.is_subscribed(event_name)
    }

    /// Get subscribed events
    pub fn subscribed_events(&self) -> Vec<String> {
        self.subscriptions.get_events()
    }

    /// Get the producer ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the client protocol name
    pub fn protocol_name(&self) -> &str {
        self.client.protocol_name()
    }

    /// Get the target address
    pub fn address(&self) -> &str {
        self.client.address()
    }
}

impl Observer for Producer {
    fn id(&self) -> &str {
        &self.id
    }

    fn notify(&self, message: &str) -> Result<()> {
        self.forward(message)
    }

    fn subscribed_events(&self) -> Vec<String> {
        self.subscriptions.get_events()
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
    fn test_producer_creation() {
        let mock_client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        let producer = Producer::new("test-producer".to_string(), mock_client);
        
        assert_eq!(producer.id(), "test-producer");
        assert_eq!(producer.protocol_name(), "MOCK");
        assert_eq!(producer.address(), "mock://test");
    }

    #[test]
    fn test_producer_forward() {
        let sent = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mock_client = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent),
        });
        let producer = Producer::new("test-producer".to_string(), mock_client);
        
        producer.forward("test message").unwrap();
        
        let messages = sent.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "test message");
    }

    #[test]
    fn test_producer_subscribe() {
        let mock_client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        let producer = Producer::new("test-producer".to_string(), mock_client);
        
        assert!(!producer.is_subscribed("event1"));
        producer.subscribe("event1");
        assert!(producer.is_subscribed("event1"));
    }

    #[test]
    fn test_producer_unsubscribe() {
        let mock_client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        let producer = Producer::new("test-producer".to_string(), mock_client);
        
        producer.subscribe("event1");
        assert!(producer.is_subscribed("event1"));
        producer.unsubscribe("event1");
        assert!(!producer.is_subscribed("event1"));
    }

    #[test]
    fn test_producer_subscribe_many() {
        let mock_client = Arc::new(MockClient {
            sent_messages: Arc::new(std::sync::Mutex::new(Vec::new())),
        });
        let producer = Producer::new("test-producer".to_string(), mock_client);
        
        producer.subscribe_many(&["event1", "event2", "event3"]);
        assert!(producer.is_subscribed("event1"));
        assert!(producer.is_subscribed("event2"));
        assert!(producer.is_subscribed("event3"));
        
        let events = producer.subscribed_events();
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_producer_observer_trait() {
        let sent = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mock_client = Arc::new(MockClient {
            sent_messages: Arc::clone(&sent),
        });
        let producer = Producer::new("test-producer".to_string(), mock_client);
        
        producer.subscribe("event1");
        producer.notify("observer message").unwrap();
        
        let messages = sent.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "observer message");
    }
}