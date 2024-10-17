use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("{0} is an invalid difficulty level")]
    InvalidDifficulty(String),

    #[error("Failed to insert recipe")]
    FailedInsertRecipe,

    #[error("Failed to update recipe")]
    FailedUpdateRecipe,

    #[error("Failed to delete recipe")]
    FailedDeleteRecipe,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let status = match self {
            CustomError::InvalidDifficulty(_) => StatusCode::BAD_REQUEST,
            CustomError::FailedInsertRecipe => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::FailedUpdateRecipe => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::FailedDeleteRecipe => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = self.to_string();

        (status, Json(error_message)).into_response()
    }
}
