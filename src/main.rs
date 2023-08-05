use anyhow::Result;

// use tracing::info;
use v3gaverse_xd::api;
use v3gaverse_xd::configuration::application::{configure, VXDServerConfiguration};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration: VXDServerConfiguration = configure().await?;

    api::web_server::start(configuration).await?;

    Ok(())
}
