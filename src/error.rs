use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CustomError {
    #[error("{0} is invalid difficulty level")]
    InvalidDifficulty(String),
    #[error("failed insert recipe")]
    FailedInsertDataRecipe,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::InvalidDifficulty(level) => (
                StatusCode::BAD_REQUEST,
                format!("{} is invalid difficulty level", level),
            ),
            CustomError::FailedInsertDataRecipe => {
                (StatusCode::BAD_REQUEST, "failed insert recipe".to_string())
            }
        };

        (status, Json(error_message)).into_response()
    }
}
