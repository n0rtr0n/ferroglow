use axum::{Router, routing::get, Json};
use ferroglow_models::{Color, Pattern, Pixel, PixelMap};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/patterns", get(patterns))
        .route("/pixel_map", get(pixel_map))
}

async fn health() -> &'static str {
    "ok"
}

async fn patterns() -> Json<Vec<Pattern>> {
    Json(vec![Pattern::new("Test Pattern".to_string())])
}

async fn pixel_map() -> Json<Vec<PixelMap>> {
    let color: Color = Color::new(0,0,0);
    Json(vec![PixelMap::new(vec![Pixel::new(0, 0, color, 0, 0)])])
}