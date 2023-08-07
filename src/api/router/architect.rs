use crate::api::web_server::ApplicationState;
use crate::data::architect::{Architect, ArchitectBody, ArchitectResponse, NewArchitect};
use crate::data::postgres_repository::PostgresRepository;
//use crate::api::app_error::{Result, AppError}; // lets use the reg Result for now
use axum::extract::{self, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use chrono::Utc;
use hyper::StatusCode;
use tracing::info;
use uuid::Uuid;

//--Architect Routes
//
pub fn router() -> Router<ApplicationState> {
    Router::new()
        //.route("/create_me", post(create_architect))
        .route("/create_me", post(create_architect))
}

pub async fn create_architect(
    State(data): State<PostgresRepository>,
    Json(req): extract::Json<NewArchitect>,
) -> (StatusCode, Json<ArchitectBody<ArchitectResponse>>) {
    let architect_id = Uuid::new_v4();
    
    let created_at = Utc::now().naive_utc();
    let architect = sqlx::query_as!(
        Architect,
        "INSERT INTO architects (id, handle, password, created_at, last_login) VALUES ($1, $2, $3, $4, $5) RETURNING *", 
        architect_id,
        req.handle,
        req.password,
        created_at,
        created_at
    )
    .fetch_one(&data.db_pool)
    .await
    .unwrap();

    info!("architect created: {:?}", architect);


    // Create a new `ArchitectResponse` struct with the handle.
    let architect_response = ArchitectResponse {
        id: architect.id,
        handle: architect.handle,
    };

    // Return the `ArchitectResponse` struct as the response.
    (
        StatusCode::OK,
        Json(ArchitectBody {
            architect: architect_response,
        }),
    )
}
