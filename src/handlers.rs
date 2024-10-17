use crate::entities::{InsertRecipe, UpdateRecipe};
use crate::error::CustomError;
use crate::helper::create_response;
use crate::interface::RecipeRepositories;
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn get_all_recipes(State(state): State<AppState>) -> impl IntoResponse {
    match state.get_recipes().await {
        Ok(recipes) => create_response(StatusCode::OK, Some(recipes), "successfully get recipes").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch recipes").into_response(),
    }
}

pub async fn get_recipe(
    State(state): State<AppState>,
    Path(recipe_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match state.get_recipe(recipe_uuid).await {
        Ok(recipe) => create_response(StatusCode::OK, Some(recipe), "successfully get recipe").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch recipe").into_response(),
    }
}

pub async fn create_recipe(
    State(state): State<AppState>,
    Json(mut payload): Json<InsertRecipe>,
) -> Result<impl IntoResponse, CustomError> {
    if let Err(_) = payload.validate() {
        return Err(CustomError::InvalidDifficulty(
            payload.difficulty_level.clone().unwrap(),
        ));
    }

    payload.uuid = Uuid::new_v4();
    payload.created_at = Utc::now();
    payload.created_by = "system".to_string();

    match state.create_recipe(payload.into()).await {
        Ok(_) => Ok(create_response(
            StatusCode::CREATED,
            None::<String>,
            "Successfully insert recipe",
        )),
        Err(_) => Err(CustomError::FailedInsertRecipe),
    }
}

pub async fn update_recipe(
    State(state): State<AppState>,
    Path(recipe_uuid): Path<Uuid>,
    Json(mut payload): Json<UpdateRecipe>,
) -> Result<impl IntoResponse, CustomError> {
    payload.uuid = recipe_uuid;
    payload.updated_at = Utc::now();
    payload.updated_by = "system".to_string();

    match state.update_recipe(payload.into()).await {
        Ok(_) => Ok(create_response(StatusCode::OK, None::<String>, "Successfully update recipe")),
        Err(_) => Err(CustomError::FailedUpdateRecipe),
    }
}

pub async fn delete_recipe(
    State(state): State<AppState>,
    Path(recipe_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, CustomError> {
    match state.delete_recipe(recipe_uuid).await {
        Ok(_) => Ok(create_response(StatusCode::OK, None::<String>, "Successfully delete recipe")),
        Err(_) => Err(CustomError::FailedDeleteRecipe),
    }
}
