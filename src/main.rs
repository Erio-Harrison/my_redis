mod network;
mod commands;
mod storage;
mod cache;
mod config;
mod logging;
mod monitoring;
mod cluster;
mod transactions;
mod pubsub;

use std::sync::{Arc, Mutex};

use commands::executor;
use storage::memory::MemoryStorage;

use crate::network::server::Server;
use crate::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match Config::from_file("config.toml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config, using default: {}", e);
            Config::new()
        }
    };

    let storage = Arc::new(Mutex::new(MemoryStorage::new()));
    let executor = Arc::new(Mutex::new(MemoryStorage::new()));

    let server = Server::new(config);
    server.run()?;

    Ok(())
}