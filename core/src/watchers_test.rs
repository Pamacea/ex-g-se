// Tests for watcher implementations

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: "test_event".to_string(),
            data: json!({"key": "value"}),
        };

        assert_eq!(entry.event_type, "test_event");
        assert_eq!(entry.data["key"], "value");
    }

    #[test]
    fn test_session_logs_creation() {
        let start = Utc::now();
        let logs = SessionLogs {
            start_time: start,
            end_time: None,
            events: vec![],
        };

        assert_eq!(logs.events.len(), 0);
        assert!(logs.end_time.is_none());
    }

    #[test]
    fn test_log_entry_serialization() {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: "fs_change".to_string(),
            data: json!({"path": "./test.txt"}),
        };

        let serialized = serde_json::to_string(&entry);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("\"type\":\"fs_change\""));
        assert!(json_str.contains("\"data\":"));
    }

    #[test]
    fn test_session_logs_serialization() {
        let logs = SessionLogs {
            start_time: Utc::now(),
            end_time: Some(Utc::now()),
            events: vec![
                LogEntry {
                    timestamp: Utc::now(),
                    event_type: "test".to_string(),
                    data: json!({}),
                },
            ],
        };

        let serialized = serde_json::to_string(&logs);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("\"start\":"));
        assert!(json_str.contains("\"end\":"));
        assert!(json_str.contains("\"events\":"));
    }

    #[test]
    fn test_multiple_events_in_session() {
        let logs = SessionLogs {
            start_time: Utc::now(),
            end_time: None,
            events: vec![
                LogEntry {
                    timestamp: Utc::now(),
                    event_type: "fs_change".to_string(),
                    data: json!({"path": "file1.txt"}),
                },
                LogEntry {
                    timestamp: Utc::now(),
                    event_type: "clipboard".to_string(),
                    data: json!({"content": "test"}),
                },
            ],
        };

        assert_eq!(logs.events.len(), 2);
        assert_eq!(logs.events[0].event_type, "fs_change");
        assert_eq!(logs.events[1].event_type, "clipboard");
    }
}
