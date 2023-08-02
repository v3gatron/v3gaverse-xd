use crate::api::web_server::ApplicationState;
use crate::data::architect::{Architect, NewArchitect};
use crate::data::postgres_repository::PostgresRepository;
use crate::error;
use axum::extract::{State, self};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use hyper::StatusCode;
use tracing::info;

// TODO: So maybe I want a ResponseArchitect here that way you have your struct not expecting a password field, etc

pub async fn create_architect(
    State(data): State<PostgresRepository>,
    Json(new_architect): extract::Json<NewArchitect>,
) -> Result<Json<Architect>, StatusCode> {
    let architect = NewArchitect::create(State(data), Json(new_architect)).await;

    info!("architect created: {:?}",architect);
    
    // NOTE: hold tight Ok(Json(Architect {id: architect.id, handle: architect.handle, password: architect.password}))
        
}

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/create_me", post(create_architect))
}
