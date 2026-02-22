// Integration tests for EX-G-SE
//
// These tests verify the complete functionality of the shadow logging system

use ex_g_se::{ExGSeEngine, SessionLogs};
use serde_json::json;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_engine_lifecycle() {
    let mut engine = ExGSeEngine::new();

    // Engine should start running
    assert!(engine.is_running());

    // Add some events
    engine.log_event("test", json!({"message": "test"}));
    assert_eq!(engine.events.len(), 1);

    // Shutdown
    engine.shutdown();
    assert!(!engine.is_running());
}

#[test]
fn test_full_session_workflow() {
    let mut engine = ExGSeEngine::new();

    // Simulate a full session with various events
    engine.log_event(
        "fs_change",
        json!({"path": "./src/main.rs", "action": "modified"}),
    );
    engine.log_event("clipboard", json!({"content": "function test() {}"}));
    engine.log_event(
        "screenshot",
        json!({"status": "captured", "file": "screen1.png"}),
    );
    engine.log_event("trigger", json!({"key": "Ctrl+Shift+X"}));

    assert_eq!(engine.events.len(), 4);

    // Verify event types
    let event_types: Vec<&str> = engine
        .events
        .iter()
        .map(|e| e.event_type.as_str())
        .collect();
    assert!(event_types.contains(&"fs_change"));
    assert!(event_types.contains(&"clipboard"));
    assert!(event_types.contains(&"screenshot"));
    assert!(event_types.contains(&"trigger"));
}

#[test]
fn test_json_output_format() {
    let mut engine = ExGSeEngine::new();
    engine.log_event("test_event", json!({"key": "value", "number": 42}));

    let logs = SessionLogs {
        start_time: engine.start_time,
        end_time: Some(chrono::Utc::now()),
        events: engine.events.clone(),
    };

    let json_output = serde_json::to_string_pretty(&logs).expect("Failed to serialize");

    // Verify JSON structure
    assert!(json_output.contains("\"start\":"));
    assert!(json_output.contains("\"end\":"));
    assert!(json_output.contains("\"events\":"));
    assert!(json_output.contains("\"type\":"));
    assert!(json_output.contains("\"ts\":"));
    assert!(json_output.contains("\"data\":"));
}

#[test]
fn test_file_system_watch_simulation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("test.txt");

    // Create a test file
    fs::write(&test_file, "test content").expect("Failed to write test file");

    let mut engine = ExGSeEngine::new();

    // Simulate FS event
    engine.log_event(
        "fs_change",
        json!({
            "path": test_file.to_string_lossy(),
            "action": "created"
        }),
    );

    assert_eq!(engine.events.len(), 1);
    assert_eq!(engine.events[0].event_type, "fs_change");
    assert!(engine.events[0].data["path"].is_string());
}

#[test]
fn test_clipboard_event_simulation() {
    let mut engine = ExGSeEngine::new();

    // Simulate clipboard events
    let clipboard_contents = vec![
        "function hello() { return 'world'; }",
        "const x = 42;",
        "https://example.com",
    ];

    for content in &clipboard_contents {
        engine.log_event("clipboard", json!({"content": content}));
    }

    assert_eq!(engine.events.len(), 3);
    for (i, event) in engine.events.iter().enumerate() {
        assert_eq!(event.event_type, "clipboard");
        assert_eq!(event.data["content"], clipboard_contents[i]);
    }
}

#[test]
fn test_screenshot_event_simulation() {
    let mut engine = ExGSeEngine::new();

    // Simulate periodic screenshots
    for i in 0..5 {
        engine.log_event(
            "screenshot",
            json!({
                "status": "captured",
                "file": format!("screenshot_{:03}.png", i),
                "timestamp": chrono::Utc::now()
            }),
        );
    }

    assert_eq!(engine.events.len(), 5);
    for event in &engine.events {
        assert_eq!(event.event_type, "screenshot");
        assert_eq!(event.data["status"], "captured");
    }
}

#[test]
fn test_trigger_shutdown() {
    let engine = ExGSeEngine::new();
    assert!(engine.is_running());

    // Simulate trigger key press
    engine.shutdown();

    assert!(!engine.is_running());
}

#[test]
fn test_high_volume_events() {
    let mut engine = ExGSeEngine::new();

    // Simulate high-volume event logging
    for i in 0..1000 {
        engine.log_event(
            "test_event",
            json!({"id": i, "data": format!("event_{}", i)}),
        );
    }

    assert_eq!(engine.events.len(), 1000);

    // Verify first and last events
    assert_eq!(engine.events[0].data["id"], 0);
    assert_eq!(engine.events[999].data["id"], 999);
}

#[test]
fn test_session_serialization_roundtrip() {
    let mut engine = ExGSeEngine::new();

    engine.log_event("event1", json!({"key": "value1"}));
    engine.log_event("event2", json!({"key": "value2"}));

    let logs = SessionLogs {
        start_time: engine.start_time,
        end_time: Some(chrono::Utc::now()),
        events: engine.events.clone(),
    };

    // Serialize
    let json_str = serde_json::to_string(&logs).expect("Failed to serialize");

    // Deserialize
    let deserialized: SessionLogs = serde_json::from_str(&json_str).expect("Failed to deserialize");

    assert_eq!(deserialized.events.len(), 2);
    assert_eq!(deserialized.events[0].event_type, "event1");
    assert_eq!(deserialized.events[1].event_type, "event2");
}

#[test]
fn test_event_data_variety() {
    let mut engine = ExGSeEngine::new();

    // Test various data types
    engine.log_event("string_test", json!({"value": "text"}));
    engine.log_event("number_test", json!({"value": 42}));
    engine.log_event("bool_test", json!({"value": true}));
    engine.log_event("null_test", json!({"value": serde_json::Value::Null}));
    engine.log_event("array_test", json!({"value": [1, 2, 3]}));
    engine.log_event("object_test", json!({"nested": {"data": "value"}}));

    assert_eq!(engine.events.len(), 6);
}
