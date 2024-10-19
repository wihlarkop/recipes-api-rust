use crate::helper::create_response;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

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

    #[error("Failed to fetch recipes")]
    FailedGetRecipes,

    #[error("Failed to fetch recipe")]
    FailedGetRecipe,

    #[error("Recipe Not Found")]
    RecipeNotFound,

    #[error("Route Not Found")]
    RouteNotFound,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let status = match self {
            CustomError::InvalidDifficulty(_) => StatusCode::BAD_REQUEST,
            CustomError::FailedInsertRecipe => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::FailedUpdateRecipe => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::FailedDeleteRecipe => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::FailedGetRecipe => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::FailedGetRecipes => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::RecipeNotFound => StatusCode::NOT_FOUND,
            CustomError::RouteNotFound => StatusCode::NOT_FOUND,
        };

        let error_message = self.to_string();

        create_response::<()>(status, None, &error_message)
    }
}

pub async fn handler_404() -> impl IntoResponse {
    Err::<(), CustomError>(CustomError::RouteNotFound)
}
