use anyhow::Result; 

// use tracing::info;
use v3gaverse_xd::configuration::application::{VXDServerConfiguration, configure};
use v3gaverse_xd::api;


#[tokio::main]
async fn main() -> Result<()> {
    
    let configuration: VXDServerConfiguration = 
        configure().await?;

    api::web_server::start(configuration).await?;
   
    Ok(())
}
 
