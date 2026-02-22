use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use chrono::Utc;

pub struct FileSystemEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub data: serde_json::Value,
}

pub fn watch_directory(path: PathBuf, tx: Sender<FileSystemEvent>) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        watch_directory_inotify(path, tx)
    }

    #[cfg(target_os = "macos")]
    {
        watch_directory_macos(path, tx)
    }

    #[cfg(target_os = "windows")]
    {
        watch_directory_windows(path, tx)
    }
}

#[cfg(target_os = "macos")]
fn watch_directory_macos(path: PathBuf, tx: Sender<FileSystemEvent>) -> Result<()> {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind, Config};
    use std::sync::mpsc::{channel, Receiver};
    use std::thread;

    let (watcher_tx, watcher_rx): (std::sync::mpsc::Sender<Result<Event, notify::Error>>, Receiver<Result<Event, notify::Error>>) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            let _ = watcher_tx.send(res);
        },
        Config::default(),
    )?;

    watcher.watch(&path, RecursiveMode::Recursive)?;

    thread::spawn(move || {
        for res in watcher_rx {
            match res {
                Ok(event) => {
                    if let Some(p) = event.paths.first() {
                        let event_type = match event.kind {
                            EventKind::Create(_) => "create",
                            EventKind::Modify(_) => "modify",
                            EventKind::Remove(_) => "remove",
                            _ => continue,
                        };

                        let output = FileSystemEvent {
                            timestamp: Utc::now(),
                            event_type: event_type.to_string(),
                            data: serde_json::json!({
                                "path": p.to_string_lossy().to_string(),
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
    });

    // Keep the thread alive - this function will block
    thread::park();
    Ok(())
}

#[cfg(target_os = "windows")]
fn watch_directory_windows(path: PathBuf, tx: Sender<FileSystemEvent>) -> Result<()> {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind, Config};
    use std::sync::mpsc::{channel, Receiver};
    use std::thread;

    let (watcher_tx, watcher_rx): (std::sync::mpsc::Sender<Result<Event, notify::Error>>, Receiver<Result<Event, notify::Error>>) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            let _ = watcher_tx.send(res);
        },
        Config::default(),
    )?;

    watcher.watch(&path, RecursiveMode::Recursive)?;

    thread::spawn(move || {
        for res in watcher_rx {
            match res {
                Ok(event) => {
                    if let Some(p) = event.paths.first() {
                        let event_type = match event.kind {
                            EventKind::Create(_) => "create",
                            EventKind::Modify(_) => "modify",
                            EventKind::Remove(_) => "remove",
                            _ => continue,
                        };

                        let output = FileSystemEvent {
                            timestamp: Utc::now(),
                            event_type: event_type.to_string(),
                            data: serde_json::json!({
                                "path": p.to_string_lossy().to_string(),
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
    });

    // Keep the thread alive - this function will block
    thread::park();
    Ok(())
}

#[cfg(target_os = "linux")]
fn watch_directory_inotify(path: PathBuf, tx: Sender<FileSystemEvent>) -> Result<()> {
    use inotify::{Inotify, WatchMask};

    let mut inotify = Inotify::init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize inotify: {}", e))?;

    inotify.add_watch(&path, WatchMask::CREATE | WatchMask::MODIFY | WatchMask::DELETE | WatchMask::MOVED_FROM | WatchMask::MOVED_TO)
        .map_err(|e| anyhow::anyhow!("Failed to watch directory: {}", e))?;

    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify.read_events_blocking(&mut buffer)
            .map_err(|e| anyhow::anyhow!("Failed to read events: {}", e))?;

        for event in &events {
            let event_type = if event.mask.contains(WatchMask::CREATE) {
                "create"
            } else if event.mask.contains(WatchMask::MODIFY) {
                "modify"
            } else if event.mask.contains(WatchMask::DELETE) {
                "remove"
            } else if event.mask.contains(WatchMask::MOVED_FROM) || event.mask.contains(WatchMask::MOVED_TO) {
                "move"
            } else {
                continue;
            };

            let file_path = if let Some(name) = event.name() {
                path.join(name.to_string_lossy().to_string())
            } else {
                path.clone()
            };

            let output = FileSystemEvent {
                timestamp: Utc::now(),
                event_type: event_type.to_string(),
                data: serde_json::json!({
                    "path": file_path.to_string_lossy(),
                }),
            };

            let _ = tx.send(output);
        }
    }
}
