use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::Result;
use axum::Router;
use tracing::info;
use axum::extract::FromRef;
use crate::api; 
use crate::configuration::application::VXDServerConfiguration;
use crate::data::postgres_repository::PostgresRepository;

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
    // let appliction_router = Router::new()
    //     .merge(api::router::health_check::router())
    //     .with_state(Arc::new(application_state));

    let socket_address: SocketAddr = SocketAddr::from((
        configuration.web_server.ipv4_address,
        configuration.web_server.port,
    ));

    info!(
        "Starting web server...\n\n [https://localhost:{}]\n",
        configuration.web_server.port
    );

    serve(
        application_router,
        socket_address,
        //        transport_layer_security_configuration,
    )
    .await?;

    Ok(())
}

pub fn api_router(app_state: ApplicationState) -> Router {
     Router::new()
        .merge(api::router::health_check::router())
        .with_state(app_state)
}

pub async fn serve(
    router: Router,
    socket_address: SocketAddr,
) -> Result<()> {
    axum_server::bind(socket_address)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

// async fn serve(
//     router: Router,
//     socket_address: SocketAddr,
//   //  transport_layer_security_configuration: RustlsConfig,
// ) -> Result<()> {
//     axum_server::bind_rustls(socket_address) //transport_layer_security_configuration)
//         .serve(router.into_make_service_with_connect_info::<SocketAddr>())
//         .await?;
//     Ok(())
// }


#[derive(Clone)]
pub struct ApplicationState {
    postgres_service: PostgresRepository,
}

impl FromRef<ApplicationState> for PostgresRepository {
    fn from_ref(state: &ApplicationState) -> Self {
        state.postgres_service.clone()
    }
}
