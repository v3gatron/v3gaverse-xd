use crate::api::web_server::ApplicationState;
use crate::data::architect::{ArchitectBody ,NewArchitect, ArchitectResponse, Architect};
use crate::data::postgres_repository::PostgresRepository;
//use crate::api::app_error::{Result, AppError}; // lets use the reg Result for now
use axum::extract::{self, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use hyper::StatusCode;
use tracing::info;

//--Architect Routes
//
pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/create_me", post(create_architect))
        .route("/create_me2", post(create_architect2))
}

//--NOTE: ArchitectResponse. change here...I think?
pub async fn create_architect(
    State(data): State<PostgresRepository>,
    Json(req): extract::Json<NewArchitect>,
) -> (StatusCode, Json<ArchitectBody<ArchitectResponse>>) {
    let architect = NewArchitect::create(State(data), Json(req)).await;
    

    // NOTE: hold tight Ok(Json(Architect {id: architect.id, handle: architect.handle, password: architect.password}))
    (StatusCode::OK,
     Json(ArchitectBody {
         architect: ArchitectResponse {
             handle: "vega".to_owned(),
         },
     }),
    )
}

pub async fn create_architect2(
    State(data): State<PostgresRepository>,
    Json(req): extract::Json<NewArchitect>,
) -> (StatusCode, Json<ArchitectBody<ArchitectResponse>>) {
    let architect = sqlx::query_as!(
        Architect,
        "INSERT INTO architects (handle, password, created_at) VALUES ($1, $2, $3) RETURNING *",
        req.handle,
        req.password,
        req.created_at
    )
    .fetch_one(&data.db_pool)
    .await
    .unwrap();

    info!("architect created: {:?}", architect);

    // Create a new `ArchitectResponse` struct with the handle.
    let architect_response = ArchitectResponse {
        handle: architect.handle,
    };

    // Return the `ArchitectResponse` struct as the response.
    (StatusCode::OK,
     Json(ArchitectBody {
         architect: architect_response,
     }),
    )
}
