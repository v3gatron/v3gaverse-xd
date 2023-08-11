pub mod api;
pub mod dto;
//pub mod error;
pub mod services;

use std::net::SocketAddr;
// use std::sync::Arc;
use crate::server::api::{architect, health_check};
use crate::configuration::application::VXDServerConfiguration;
use crate::data::postgres_repository::PostgresRepository;
use crate::web;
use anyhow::Result;
use axum::extract::FromRef;
//use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn start(configuration: VXDServerConfiguration) -> Result<()> {
    // TLS?

    info!("Attemping to establish a database connect...");
    let postgres_service: PostgresRepository =
        PostgresRepository::new(configuration.database.postgresql_connection_string).await?;
    info!("Connection established.");

    //let formatted_url: String = format!("https://localhost:{}", configuration.web_server.port);

    let application_state: ApplicationState = ApplicationState {
        postgres_service, // NOTE: I think this may be where you want an arc?  No you want to pass
                          // application state as an Arc
                          //encryption_service,
                          //web_authentication_service: Arc::new(web_authentication_service),
                          //key: Key::from(configuration.encryption.site_secret.as_bytes()),
    };

    let application_router = api_router(application_state);

    let socket_address: SocketAddr = SocketAddr::from((
        configuration.web_server.ipv4_address,
        configuration.web_server.port,
    ));

    info!(
        "Starting web server...\n\n [https://localhost:{}]\n",
        configuration.web_server.port
    );
    info!("👾🦋 Böötes Void stirred, and the v3gaverse burst into being!!!");

    serve(
        application_router,
        socket_address,
        // transport_layer_security_configuration,
    )
    .await?;

    Ok(())
}

pub fn api_router(app_state: ApplicationState) -> Router {
    Router::new()
        .merge(web::router())
        .merge(architect::router())
        .merge(health_check::router())
        .layer(TraceLayer::new_for_http())
        .nest_service("/resources", ServeDir::new("resources"))
        .with_state(app_state)
}

pub async fn serve(router: Router, socket_address: SocketAddr) -> Result<()> {
    axum_server::bind(socket_address)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

#[derive(Clone)]
pub struct ApplicationState {
    pub postgres_service: PostgresRepository,
}

impl FromRef<ApplicationState> for PostgresRepository {
    fn from_ref(state: &ApplicationState) -> Self {
        state.postgres_service.clone()
    }
}
