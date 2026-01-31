mod config;
mod matcher;
mod mock;
mod proxy;

use clap::Parser;
use config::Config;
use proxy::ProxyServer;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,

    /// Host to bind to
    #[arg(long)]
    host: Option<String>,

    /// Port to bind to
    #[arg(short, long)]
    port: Option<u16>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose {
        "gambas_matcher_mock=debug,info"
    } else {
        "gambas_matcher_mock=info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    log::info!("Gambas Matcher Mock - API Interception and Simulation Utility");

    // Load configuration
    let mut config = if args.config.exists() {
        log::info!("Loading configuration from: {:?}", args.config);
        Config::from_file(&args.config)?
    } else {
        log::warn!(
            "Configuration file not found: {:?}, using defaults",
            args.config
        );
        Config::default()
    };

    // Override config with CLI arguments if provided
    if let Some(host) = args.host {
        config.server.host = host;
    }
    if let Some(port) = args.port {
        config.server.port = port;
    }

    // Create and run proxy server
    let server = ProxyServer::new(config);
    server.run().await?;

    Ok(())
}
