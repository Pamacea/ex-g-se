// EX-G-SE: Shadow Logging Core Engine - Library Interface
//
// This library provides the core functionality for the EX-G-SE shadow logging system.

pub mod core;
pub mod watchers;

pub use core::ExGSeEngine;
pub use watchers::{LogEntry, SessionLogs};

// Include test modules
#[cfg(test)]
mod tests {
    // Tests are in separate files to keep the main code clean
    // The test modules will be compiled when running `cargo test`
}
