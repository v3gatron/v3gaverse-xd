use anyhow::Result; 

pub mod configuration;
use axum::{response::Html, Router, routing::get};
use configuration::application::VXDServerConfiguration;
use v3gaverse_xd::api;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Böötes Void stirred, and the v3gaverse burst into being!!! 👻👾");
    //configuration start api::web_server::start(configuration).await?;
    let configuration: VXDServerConfiguration = 
        configuration::application::configure().await?;

    //  let configuration: VXDWebServerConfiguration = match configuration::application::configure().await {
    //     Ok(configuration) => configuration,
    //     Err(error) => panic!("Failed to configure application: {}", error),
    // };


    api::web_server::start(configuration).await?;
    // let app = Router::new().route("/", get(handler)); 
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
