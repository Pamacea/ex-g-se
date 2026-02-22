use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::output::OutputEvent;
use chrono::Utc;

pub fn watch_directory(path: PathBuf, tx: Sender<OutputEvent>) -> Result<()> {
    let (watcher_tx, watcher_rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            let _ = watcher_tx.send(res);
        },
        Duration::from_millis(200),
    )?;

    watcher.watch(&path, RecursiveMode::Recursive)?;

    for res in watcher_rx {
        match res {
            Ok(event) => {
                if let Some(path) = event.paths.first() {
                    let event_type = match event.kind {
                        EventKind::Create(_) => "create",
                        EventKind::Modify(_) => "modify",
                        EventKind::Remove(_) => "remove",
                        _ => continue,
                    };

                    let output = OutputEvent::FileSystem {
                        timestamp: Utc::now(),
                        event: event_type.to_string(),
                        path: path.to_string_lossy().to_string(),
                        size: None, // Could add file size fetching here
                    };

                    let _ = tx.send(output);
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
            }
        }
    }

    Ok(())
}
