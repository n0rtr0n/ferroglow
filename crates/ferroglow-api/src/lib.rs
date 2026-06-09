use axum::{Router, routing::get, Json};
use ferroglow_models::Pattern;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/patterns", get(patterns))
}

async fn health() -> &'static str {
    "ok"
}

async fn patterns() -> Json<Vec<Pattern>> {
    Json(vec![Pattern::new("Test Pattern".to_string())])
}