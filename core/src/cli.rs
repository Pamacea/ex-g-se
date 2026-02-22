use std::env;

/// CLI configuration from command line arguments
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub max_duration_secs: Option<u64>,
    pub max_events: Option<usize>,
    pub session_label: Option<String>,
    pub tags: Vec<String>,
}

impl CliConfig {
    /// Parse CLI arguments
    /// Supports:
    ///   --duration <minutes>   Auto-stop after N minutes
    ///   --max-events <n>       Stop after N events
    ///   --label <text>         Session label
    ///   --tags <tag1,tag2>     Session tags
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut config = CliConfig {
            max_duration_secs: None,
            max_events: None,
            session_label: None,
            tags: Vec::new(),
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--duration" => {
                    if i + 1 < args.len() {
                        if let Ok(minutes) = args[i + 1].parse::<u64>() {
                            config.max_duration_secs = Some(minutes * 60);
                        }
                        i += 1;
                    }
                }
                "--max-events" => {
                    if i + 1 < args.len() {
                        if let Ok(count) = args[i + 1].parse::<usize>() {
                            config.max_events = Some(count);
                        }
                        i += 1;
                    }
                }
                "--label" => {
                    if i + 1 < args.len() {
                        config.session_label = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--tags" => {
                    if i + 1 < args.len() {
                        config.tags = args[i + 1]
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        config
    }

    /// Check if we should stop based on limits
    pub fn should_stop(&self, elapsed_secs: i64, event_count: usize) -> bool {
        if let Some(max_duration) = self.max_duration_secs {
            if elapsed_secs >= max_duration as i64 {
                return true;
            }
        }

        if let Some(max_events) = self.max_events {
            if event_count >= max_events {
                return true;
            }
        }

        false
    }
}
