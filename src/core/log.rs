//! Logging (tracing, log, env_logger)

use tracing_subscriber;

pub fn init_logger() {
    // Use tracing with env_logger as default
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    // Or, for legacy log:
    // env_logger::init();
}

// Example usage: tracing::info!("Server started!");
