use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::{fmt, fs};
//use tracing::info;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub const VXD_CONFIG_FILE: &str = "./v3gaverse-config.ron";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VXDServerConfiguration {
    pub environment: Environment,
    pub web_server: WebServer,
    pub database: Database,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Local,
    Development,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebServer {
    pub port: u16,
    pub ipv4_address: [u8; 4], // NOTE: Can't you use a ipv4/ipv6 type here? also does that guarantee more security?
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Database {
    pub postgresql_connection_string: String,
}

pub async fn configure() -> Result<VXDServerConfiguration> {
    info!("Reading configuration from `{}` file.", VXD_CONFIG_FILE);
    let file_content: String = fs::read_to_string(VXD_CONFIG_FILE)?;
    let configuration: VXDServerConfiguration = ron::from_str(&file_content)?;
    println!("{}", file_content);

    // TODO: Add tracing in here?
    configure_tracing(&configuration);
    Ok(configuration)
}

pub fn configure_tracing(configuration: &VXDServerConfiguration) {
    let environment: String = configuration.environment.to_string().to_ascii_uppercase();

    let app_name = format!("VXD_{}", environment);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("info,{}=debug", app_name)));

    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    info!("Tracing is configured.");
}
