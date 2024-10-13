use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub fn create_response(status: StatusCode, message: &str) -> impl IntoResponse {
    (status, Json(json!({ "message": message }))).into_response()
}
