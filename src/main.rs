use anyhow::Result;

// use tracing::info;
use v3gaverse_xd::server::start;
use v3gaverse_xd::configuration::application::{configure, VXDServerConfiguration};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration: VXDServerConfiguration = configure().await?;

    start(configuration).await?;

    Ok(())
}
