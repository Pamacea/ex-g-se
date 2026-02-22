use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::output::OutputEvent;
use chrono::Utc;

#[cfg(not(target_os = "linux"))]
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};

#[cfg(target_os = "linux")]
use inotify::{Inotify, WatchMask};

pub fn watch_directory(path: PathBuf, tx: Sender<OutputEvent>) -> Result<()> {
    #[cfg(not(target_os = "linux"))]
    {
        watch_directory_notify(path, tx)
    }

    #[cfg(target_os = "linux")]
    {
        watch_directory_inotify(path, tx)
    }
}

#[cfg(not(target_os = "linux"))]
fn watch_directory_notify(path: PathBuf, tx: Sender<OutputEvent>) -> Result<()> {
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

                    let output = OutputEvent {
                        timestamp: Utc::now(),
                        event_type: event_type.to_string(),
                        data: serde_json::json!({
                            "path": path.to_string_lossy(),
                        }),
                    };

                    let _ = tx.send(output);
                }
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
            }
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn watch_directory_inotify(path: PathBuf, tx: Sender<OutputEvent>) -> Result<()> {
    let mut inotify = Inotify::init()?;

    inotify.add_watch(&path, WatchMask::CREATE | WatchMask::MODIFY | WatchMask::DELETE)?;

    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify.read_events_blocking(&mut buffer)?;

        for event in events {
            let event_type = if event.mask.contains(WatchMask::CREATE) {
                "create"
            } else if event.mask.contains(WatchMask::MODIFY) {
                "modify"
            } else if event.mask.contains(WatchMask::DELETE) {
                "remove"
            } else {
                continue;
            };

            let output = OutputEvent {
                timestamp: Utc::now(),
                event_type: event_type.to_string(),
                data: serde_json::json!({
                    "path": path.join(event.name.unwrap_or_else(|| std::ffi::OsStr::new("")).to_string_lossy()).to_string_lossy(),
                }),
            };

            let _ = tx.send(output);
        }
    }
}
