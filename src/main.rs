use anyhow::Result;
use clap::Parser;

mod app;
mod ui;
mod llm;
mod tools;
mod context;
mod config;
mod agent;
mod commands;
mod message;
mod logging;

#[derive(Parser)]
#[command(name = "sree")]
#[command(version = "0.1.0")]
#[command(about = "A production-grade AI CLI assistant", long_about = None)]
struct Cli {
    /// Path to custom config file (default: ~/.sree/config.toml)
    #[arg(long)]
    config: Option<String>,
    
    /// Override model (e.g., anthropic.claude-sonnet-4-20250514-v1:0)
    #[arg(long)]
    model: Option<String>,
    
    /// Override AWS region (e.g., us-east-1, us-west-2)
    #[arg(long)]
    region: Option<String>,
    
    /// AWS profile to use (from ~/.aws/credentials)
    #[arg(long)]
    profile: Option<String>,
    
    /// Disable tool use (chat-only mode)
    #[arg(long)]
    no_tools: bool,
    
    /// Set custom system prompt
    #[arg(long)]
    system: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    if let Err(e) = logging::init() {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }
    
    tracing::info!("sree starting up");
    
    // Set panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tracing::error!("Panic occurred: {:?}", panic_info);
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        original_hook(panic_info);
    }));
    
    let cli = Cli::parse();
    
    tracing::debug!("Creating app instance with CLI overrides");
    
    // Create and run app with CLI overrides
    let mut app = app::App::with_cli_overrides(
        cli.config, 
        cli.model, 
        cli.region, 
        cli.profile, 
        cli.no_tools, 
        cli.system
    ).await;
    
    // Set up signal handling
    let result = tokio::select! {
        res = app.run() => res,
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received SIGINT, shutting down gracefully");
            app.shutdown();
            Ok(())
        }
        _ = async {
            #[cfg(unix)]
            {
                let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
                sigterm.recv().await;
                Ok::<(), std::io::Error>(())
            }
            #[cfg(not(unix))]
            {
                std::future::pending::<()>().await;
                Ok::<(), std::io::Error>(())
            }
        } => {
            tracing::info!("Received SIGTERM, shutting down gracefully");
            app.shutdown();
            Ok(())
        }
    };
    
    tracing::info!("sree shutting down");
    
    result
}
