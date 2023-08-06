use axum::{routing::get, Router};

use crate::api::web_server::ApplicationState;

pub mod cyberdeck;
pub mod homepage;

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/", get(homepage::homepage))
        .route("/cyberdeck", get(cyberdeck::homepage))
}
