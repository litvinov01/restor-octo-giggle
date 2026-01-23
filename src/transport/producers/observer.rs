use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Observer trait for producers that can subscribe to events
pub trait Observer: Send + Sync {
    /// Get the observer ID
    fn id(&self) -> &str;
    
    /// Notify the observer with a message
    fn notify(&self, message: &str) -> std::io::Result<()>;
    
    /// Get subscribed events
    fn subscribed_events(&self) -> Vec<String>;
}

/// Event subscriptions for an observer
pub struct EventSubscriptions {
    events: Arc<Mutex<HashSet<String>>>,
}

impl EventSubscriptions {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Subscribe to an event
    pub fn subscribe(&self, event_name: &str) {
        let mut events = self.events.lock().unwrap();
        events.insert(event_name.to_string());
    }

    /// Unsubscribe from an event
    pub fn unsubscribe(&self, event_name: &str) {
        let mut events = self.events.lock().unwrap();
        events.remove(event_name);
    }

    /// Check if subscribed to an event
    pub fn is_subscribed(&self, event_name: &str) -> bool {
        let events = self.events.lock().unwrap();
        events.contains(event_name)
    }

    /// Get all subscribed events
    pub fn get_events(&self) -> Vec<String> {
        let events = self.events.lock().unwrap();
        events.iter().cloned().collect()
    }

    /// Subscribe to multiple events
    pub fn subscribe_many(&self, event_names: &[&str]) {
        let mut events = self.events.lock().unwrap();
        for event in event_names {
            events.insert(event.to_string());
        }
    }

    /// Clear all subscriptions
    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }
}

impl Default for EventSubscriptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_subscriptions_creation() {
        let subs = EventSubscriptions::new();
        assert_eq!(subs.get_events().len(), 0);
    }

    #[test]
    fn test_event_subscriptions_subscribe() {
        let subs = EventSubscriptions::new();
        subs.subscribe("event1");
        assert!(subs.is_subscribed("event1"));
        assert!(!subs.is_subscribed("event2"));
    }

    #[test]
    fn test_event_subscriptions_unsubscribe() {
        let subs = EventSubscriptions::new();
        subs.subscribe("event1");
        assert!(subs.is_subscribed("event1"));
        subs.unsubscribe("event1");
        assert!(!subs.is_subscribed("event1"));
    }

    #[test]
    fn test_event_subscriptions_subscribe_many() {
        let subs = EventSubscriptions::new();
        subs.subscribe_many(&["event1", "event2", "event3"]);
        assert!(subs.is_subscribed("event1"));
        assert!(subs.is_subscribed("event2"));
        assert!(subs.is_subscribed("event3"));
        
        let events = subs.get_events();
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_event_subscriptions_clear() {
        let subs = EventSubscriptions::new();
        subs.subscribe_many(&["event1", "event2", "event3"]);
        assert_eq!(subs.get_events().len(), 3);
        subs.clear();
        assert_eq!(subs.get_events().len(), 0);
    }

    #[test]
    fn test_event_subscriptions_duplicate_subscribe() {
        let subs = EventSubscriptions::new();
        subs.subscribe("event1");
        subs.subscribe("event1"); // Duplicate
        let events = subs.get_events();
        assert_eq!(events.len(), 1); // Should still be 1
    }
}