mod server;
mod client;
mod metrics;
mod anomaly;
mod alerts;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name="lsrm", version, about="Linux System Resource Monitor")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the gRPC server
    Server {
        /// Address to bind (e.g., 0.0.0.0:50051)
        #[arg(long, default_value = "0.0.0.0:50051")]
        addr: String,
        /// Sampling interval in milliseconds
        #[arg(long, default_value_t = 1000)]
        interval_ms: u64,
        /// Number of top processes to include
        #[arg(long, default_value_t = 5)]
        top_n: usize,
    },
    /// Run a simple client (once or stream)
    Client {
        /// Server address (e.g., http://127.0.0.1:50051)
        #[arg(long, default_value = "http://127.0.0.1:50051")]
        addr: String,
        /// Stream metrics continuously
        #[arg(long)]
        stream: bool,
        /// Output JSON lines instead of pretty text
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();
    match cli.cmd {
        Command::Server { addr, interval_ms, top_n } => {
            server::run(addr, interval_ms, top_n).await?;
        }
        Command::Client { addr, stream, json } => {
            client::run(addr, stream, json).await?;
        }
    }
    Ok(())
}
