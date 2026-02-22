use anyhow::Result;
use std::sync::mpsc::Sender;
use crate::output::OutputEvent;
use chrono::Utc;

pub fn monitor_keyboard(tx: Sender<OutputEvent>) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use rdev::{listen, Event, EventType, Key};

        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let output = OutputEvent::Keyboard {
                        timestamp: Utc::now(),
                        action: "keypress".to_string(),
                        key: format!("{:?}", key),
                        modifiers: vec![],
                    };
                    let _ = tx.send(output);
                }
                _ => {}
            }
        };

        if let Err(e) = listen(callback) {
            eprintln!("Keyboard listen error: {:?}", e);
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Stub implementation for other platforms
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    Ok(())
}
