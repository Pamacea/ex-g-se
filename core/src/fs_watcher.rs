use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::time::{Duration, Instant};
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
    use std::thread;

    thread::spawn(move || {
        let mut file_states: HashMap<String, (u64, Instant)> = HashMap::new();
        let mut first_scan = true;

        eprintln!("[FS] Windows file watcher started (polling mode)");

        loop {
            thread::sleep(Duration::from_secs(2));

            // Walk directory recursively and check for changes
            let walk_result = walk_directory(&path, &mut file_states, &mut first_scan, &tx);

            if let Err(e) = walk_result {
                eprintln!("[FS] Watch error: {}", e);
            }
        }
    });

    // Keep function alive but don't block
    thread::park();
    Ok(())
}

#[cfg(target_os = "windows")]
fn walk_directory(
    path: &PathBuf,
    file_states: &mut HashMap<String, (u64, Instant)>,
    first_scan: &mut bool,
    tx: &Sender<FileSystemEvent>
) -> Result<()> {
    use std::fs;

    let entries = fs::read_dir(path)
        .map_err(|e| anyhow::anyhow!("Failed to read directory {}: {}", path.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| anyhow::anyhow!("Failed to read entry: {}", e))?;
        let file_path = entry.path();
        let path_str = file_path.to_string_lossy().to_string();

        // Skip common ignores
        if path_str.contains("node_modules")
            || path_str.contains(".git")
            || path_str.contains("target")
            || path_str.contains("dist")
            || path_str.contains(".ex-g-se")
            || path_str.contains(".next")
            || path_str.contains("coverage")
        {
            continue;
        }

        // Skip dot files/directories
        if let Some(name) = file_path.file_name() {
            if let Some(name_str) = name.to_str() {
                if name_str.starts_with('.') && name_str != ".gitignore" && name_str != ".env" {
                    continue;
                }
            }
        }

        let metadata = entry.metadata()
            .map_err(|e| anyhow::anyhow!("Failed to get metadata: {}", e))?;

        // Recursively scan subdirectories
        if metadata.is_dir() {
            walk_directory(&file_path, file_states, first_scan, tx)?;
            continue;
        }

        // Only process files
        if !metadata.is_file() {
            continue;
        }

        // Skip binary files and logs
        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if matches!(ext_str,
                    "exe" | "dll" | "so" | "dylib" | "png" | "jpg" | "jpeg" | "gif" | "ico" |
                    "pdf" | "zip" | "tar" | "gz" | "log" | "tmp" | "lock" | "sqlite" | "db"
                ) {
                    continue;
                }
            }
        }

        // Get modification time
        let modified = metadata.modified()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let modified_secs = modified.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let last_modified = file_states.get(&path_str)
            .map(|(secs, _)| *secs)
            .unwrap_or(0);

        // File was modified (and this isn't the first scan)
        if modified_secs > last_modified && !*first_scan {
            let now = Instant::now();

            // Debounce: don't fire within 3 seconds of last event
            let should_send = if let Some((_, last_time)) = file_states.get(&path_str) {
                now.duration_since(*last_time).as_secs() >= 3
            } else {
                true
            };

            if should_send {
                file_states.insert(path_str.clone(), (modified_secs, now));

                let event = FileSystemEvent {
                    timestamp: chrono::Utc::now(),
                    event_type: "modify".to_string(),
                    data: serde_json::json!({
                        "path": path_str,
                        "size": metadata.len(),
                    }),
                };

                let _ = tx.send(event);
                eprintln!("[FS] File changed: {}", path_str);
            }
        }

        // Update state
        if *first_scan || modified_secs > last_modified {
            file_states.insert(path_str, (modified_secs, Instant::now()));
        }
    }

    // Mark first scan complete after initial traversal
    if *first_scan {
        *first_scan = false;
    }

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
