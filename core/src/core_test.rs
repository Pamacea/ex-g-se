// Tests for core engine

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_engine_creation() {
        let engine = ExGSeEngine::new();
        assert_eq!(engine.events.len(), 0);
        assert!(engine.is_running());
    }

    #[test]
    fn test_engine_default() {
        let engine = ExGSeEngine::default();
        assert_eq!(engine.events.len(), 0);
        assert!(engine.is_running());
    }

    #[test]
    fn test_log_event() {
        let mut engine = ExGSeEngine::new();
        engine.log_event("test_event", json!({"key": "value"}));

        assert_eq!(engine.events.len(), 1);
        assert_eq!(engine.events[0].event_type, "test_event");
        assert_eq!(engine.events[0].data["key"], "value");
    }

    #[test]
    fn test_log_multiple_events() {
        let mut engine = ExGSeEngine::new();

        engine.log_event("fs_change", json!({"path": "file1.txt"}));
        engine.log_event("clipboard", json!({"content": "test"}));
        engine.log_event("screenshot", json!({"status": "captured"}));

        assert_eq!(engine.events.len(), 3);
        assert_eq!(engine.events[0].event_type, "fs_change");
        assert_eq!(engine.events[1].event_type, "clipboard");
        assert_eq!(engine.events[2].event_type, "screenshot");
    }

    #[test]
    fn test_shutdown() {
        let engine = ExGSeEngine::new();
        assert!(engine.is_running());

        engine.shutdown();
        assert!(!engine.is_running());
    }

    #[test]
    fn test_timestamp_ordering() {
        let mut engine = ExGSeEngine::new();

        engine.log_event("event1", json!({}));
        std::thread::sleep(std::time::Duration::from_millis(10));
        engine.log_event("event2", json!({}));

        assert!(engine.events[1].timestamp > engine.events[0].timestamp);
    }

    #[test]
    fn test_event_data_preservation() {
        let mut engine = ExGSeEngine::new();

        let complex_data = json!({
            "string": "value",
            "number": 42,
            "bool": true,
            "null": null,
            "array": [1, 2, 3],
            "object": {"nested": "data"}
        });

        engine.log_event("complex", complex_data.clone());

        assert_eq!(engine.events[0].data, complex_data);
    }

    #[test]
    fn test_concurrent_logging() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let engine = Arc::new(Mutex::new(ExGSeEngine::new()));
        let mut handles = vec![];

        for i in 0..10 {
            let engine_clone = Arc::clone(&engine);
            let handle = thread::spawn(move || {
                let mut engine = engine_clone.lock().unwrap();
                engine.log_event(&format!("thread_{}", i), json!({"id": i}));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let engine = engine.lock().unwrap();
        assert_eq!(engine.events.len(), 10);
    }
}
