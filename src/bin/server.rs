use kv_db::{server, DEFAULT_PORT};

use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    // enable logging
    // see https://docs.rs/tracing for more info
    tracing_subscriber::fmt::try_init()?;

    let cli = Cli::from_args();
    let port = cli.port.as_deref().unwrap_or(DEFAULT_PORT);

    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct Cli {
    #[structopt(name = "port", long = "--port")]
    port: Option<String>,
}
