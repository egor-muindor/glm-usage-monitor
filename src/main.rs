//! GLM Usage Monitor - Realtime GLM Coding Plan usage monitor with TUI

#![allow(clippy::doc_markdown)]

mod app;
mod api;
mod cache;
mod config;
mod models;
mod terminal;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;
use std::time::Duration;

/// GLM Usage Monitor - Realtime GLM Coding Plan usage monitor with TUI
#[derive(Debug, Parser)]
struct Cli {
    /// Override refresh interval in seconds (default: from ENV or 300)
    #[arg(short, long)]
    refresh_sec: Option<u64>,

    /// Override HTTP timeout in seconds (default: from ENV or 20)
    #[arg(short, long)]
    timeout_sec: Option<u64>,

    /// Tick rate for the UI in milliseconds (default: 250)
    #[arg(long, default_value_t = 250)]
    tick_rate: u64,

    /// Output compact status for status bar (uses cache with 5-min TTL)
    #[arg(long)]
    status: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let mut config = config::Config::load()
        .context("Failed to load configuration. Please ensure ANTHROPIC_BASE_URL and ANTHROPIC_AUTH_TOKEN are set, or create a config file at ~/.config/glm-usage-monitor/config.toml")?;

    // Apply CLI overrides
    if let Some(refresh) = cli.refresh_sec {
        config.refresh_sec = refresh;
    }
    if let Some(timeout) = cli.timeout_sec {
        config.http_timeout_sec = timeout;
    }

    // Handle --status mode
    if cli.status {
        return run_status_mode(&config).await;
    }

    // Create application (loads cached data if available)
    let mut app = app::App::new(config)
        .context("Failed to initialize application")?;

    // Run TUI
    let tick_rate = Duration::from_millis(cli.tick_rate);
    terminal::run(&mut app, tick_rate)
        .await
        .context("Failed to run TUI")?;

    Ok(())
}

const CACHE_TTL_SECS: u64 = 300; // 5 minutes

async fn run_status_mode(config: &config::Config) -> Result<()> {
    // 1. Check cache
    if let Some(entry) = cache::read_cache() {
        if cache::is_fresh(&entry, CACHE_TTL_SECS) {
            // Use cached data
            println!("{}", models::format_status_line(&entry.data, config.time_threshold));
            return Ok(());
        }
    }

    // 2. Fetch fresh data from API
    let endpoints = config.endpoints()?;
    let client = api::GlmApiClient::new(config.auth_token.clone(), endpoints, config.http_timeout_sec);

    match client.fetch_quota_limit().await {
        Ok(data) => {
            // Save to cache
            cache::write_cache(&data);
            // Print status line
            println!("{}", models::format_status_line(&data, config.time_threshold));
        }
        Err(e) => {
            eprintln!("⚠️ GLM: error");
            return Err(e);
        }
    }

    Ok(())
}
