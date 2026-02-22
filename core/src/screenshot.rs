use anyhow::Result;
use std::sync::mpsc::Sender;
use crate::output::OutputEvent;
use chrono::Utc;

pub fn capture_screenshots(interval_ms: u64, tx: Sender<OutputEvent>) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use screenshots::Screen;
        let duration = std::time::Duration::from_millis(interval_ms);

        loop {
            if let Ok(screens) = Screen::all() {
                if let Some(screen) = screens.first() {
                    if let Ok(image) = screen.capture() {
                        let timestamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();
                        let path = format!(".ex-g-se/screenshots/{}.png", timestamp);

                        // In production, would save the image to disk
                        let output = OutputEvent::Screenshot {
                            timestamp: Utc::now(),
                            path,
                            width: image.width(),
                            height: image.height(),
                            size: 0, // Would be actual file size
                        };
                        let _ = tx.send(output);
                    }
                }
            }
            std::thread::sleep(duration);
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Stub implementation for other platforms
        let duration = std::time::Duration::from_millis(interval_ms);
        loop {
            std::thread::sleep(duration);
        }
    }

    Ok(())
}
