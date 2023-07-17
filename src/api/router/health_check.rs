use axum::{Router, routing::get};

use crate::api::web_server::ApplicationState;

pub async fn health_check() -> String {
    "hello there 22, now".to_owned()
}


pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/health_check", get(health_check))
}
