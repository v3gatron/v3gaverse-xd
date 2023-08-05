use crate::api::web_server::ApplicationState;
use crate::data::architect::{ArchitectBody ,NewArchitect, ArchitectResponse};
use crate::data::postgres_repository::PostgresRepository;
// use crate::api::app_error::{Result, AppError};
use anyhow::Error;
use axum::extract::{self, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use chrono::{NaiveDateTime, Utc, Local};
use hyper::StatusCode;
use tracing::info;

//--Architect Routes
//
pub fn router() -> Router<ApplicationState> {
    Router::new().route("/create_me", post(create_architect))
}

//--NOTE: ArchitectResponse. change here...I think?
#[axum::debug_handler]
pub async fn create_architect(
    State(data): State<PostgresRepository>,
    Json(req): extract::Json<NewArchitect>,
) -> (StatusCode, Json<ArchitectBody<ArchitectResponse>>) {
    let architect = NewArchitect::create(State(data), Json(req)).await;

    info!("architect created: {:?}", architect);

    // NOTE: hold tight Ok(Json(Architect {id: architect.id, handle: architect.handle, password: architect.password}))
    (
        StatusCode::OK,
        Json(ArchitectBody {
            architect: ArchitectResponse {
                handle: "vega".to_owned(),
            },
        }),
    )
}

