use serde::{Deserialize, Serialize};

/// Message structure with event name for routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    /// The actual message content
    pub msg: String,
    /// Event name for routing to subscribed producers
    pub event_name: String,
}

impl EventMessage {
    /// Create a new event message
    pub fn new(msg: String, event_name: String) -> Self {
        Self { msg, event_name }
    }

    /// Parse message from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse from simple format: "event_name:message"
    pub fn from_simple_format(text: &str) -> Self {
        if let Some((event, msg)) = text.splitn(2, ':').collect::<Vec<_>>().split_first() {
            let msg_part = msg.join(":");
            if msg_part.is_empty() {
                // No message part, treat whole string as message
                Self {
                    event_name: "default".to_string(),
                    msg: text.to_string(),
                }
            } else {
                Self {
                    event_name: event.trim().to_string(),
                    msg: msg_part.trim().to_string(),
                }
            }
        } else {
            // Default event if no separator
            Self {
                event_name: "default".to_string(),
                msg: text.to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_message_creation() {
        let msg = EventMessage::new("Hello".to_string(), "test_event".to_string());
        assert_eq!(msg.msg, "Hello");
        assert_eq!(msg.event_name, "test_event");
    }

    #[test]
    fn test_event_message_json_serialization() {
        let msg = EventMessage::new("Hello".to_string(), "test_event".to_string());
        let json = msg.to_json().unwrap();
        assert!(json.contains("Hello"));
        assert!(json.contains("test_event"));
    }

    #[test]
    fn test_event_message_json_deserialization() {
        let json = r#"{"msg":"Hello","event_name":"test_event"}"#;
        let msg = EventMessage::from_json(json).unwrap();
        assert_eq!(msg.msg, "Hello");
        assert_eq!(msg.event_name, "test_event");
    }

    #[test]
    fn test_event_message_simple_format() {
        let msg = EventMessage::from_simple_format("test_event:Hello World");
        assert_eq!(msg.event_name, "test_event");
        assert_eq!(msg.msg, "Hello World");
    }

    #[test]
    fn test_event_message_simple_format_no_separator() {
        let msg = EventMessage::from_simple_format("Hello World");
        assert_eq!(msg.msg, "Hello World");
        assert_eq!(msg.event_name, "default");
    }

    #[test]
    fn test_event_message_simple_format_multiple_colons() {
        let msg = EventMessage::from_simple_format("test_event:Hello:World:Test");
        assert_eq!(msg.event_name, "test_event");
        assert_eq!(msg.msg, "Hello:World:Test");
    }

    #[test]
    fn test_event_message_roundtrip_json() {
        let original = EventMessage::new("Test Message".to_string(), "my_event".to_string());
        let json = original.to_json().unwrap();
        let parsed = EventMessage::from_json(&json).unwrap();
        assert_eq!(original.msg, parsed.msg);
        assert_eq!(original.event_name, parsed.event_name);
    }
}