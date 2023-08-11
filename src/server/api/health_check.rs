use axum::{routing::get, Router};

use crate::server::ApplicationState;

pub async fn health_check() -> String {
    "The answer is is still 42".to_owned()
}

pub fn router() -> Router<ApplicationState> {
    Router::new().route("/health_check", get(health_check))
}
