use crate::entities::{InsertRecipe, Recipe, UpdateRecipe};
use crate::error::CustomError;
use crate::helper::create_response;
use crate::interface::RecipeRepositories;
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

pub async fn get_all_recipes(State(state): State<AppState>) -> impl IntoResponse {
    match query_as!(Recipe, r#"SELECT * from recipes where deleted_at is null"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(recipes) => (StatusCode::OK, Json(recipes)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch recipes").into_response(),
    }
}

pub async fn get_recipe(
    State(state): State<AppState>,
    Path(recipe_uuid): Path<Uuid>,
) -> impl IntoResponse {
    match state.get_recipe(recipe_uuid).await {
        Ok(recipe) => (StatusCode::OK, Json(recipe)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch recipe").into_response(),
    }
}

pub async fn create_recipe(
    State(state): State<AppState>,
    Json(mut payload): Json<InsertRecipe>,
) -> impl IntoResponse {
    if let Err(_) = payload.validate() {
        return Err(CustomError::InvalidDifficulty(
            payload.difficulty_level.unwrap(),
        ));
    }

    payload.uuid = Uuid::new_v4();
    payload.created_at = Utc::now();
    payload.created_by = "system".to_string();

    let result = query!(
        r#"
        INSERT INTO recipes (uuid, name, description, ingredients, instructions, prep_time, cook_time, servings, difficulty_level, created_by, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        payload.uuid,
        payload.name,
        payload.description,
        payload.ingredients,
        payload.instructions,
        payload.prep_time,
        payload.cook_time,
        payload.servings,
        payload.difficulty_level,
        payload.created_by,
        payload.created_at
    ).execute(&state.db).await;

    match result {
        Ok(_) => Ok(create_response(
            StatusCode::CREATED,
            "Successfully insert recipe",
        )),
        Err(_) => Err(CustomError::FailedInsertDataRecipe),
    }
}

pub async fn update_recipe(
    State(state): State<AppState>,
    Path(recipe_uuid): Path<Uuid>,
    Json(mut payload): Json<UpdateRecipe>,
) -> impl IntoResponse {
    payload.updated_at = Utc::now();
    payload.updated_by = "system".to_string();

    let result = query!(
        r#"
        UPDATE recipes
        SET
            name = $1,
            description = $2,
            ingredients = $3,
            instructions = $4,
            prep_time = $5,
            cook_time = $6,
            servings = $7,
            difficulty_level = $8,
            updated_by = $9,
            updated_at = $10
        WHERE uuid = $11
        "#,
        payload.name,
        payload.description,
        payload.ingredients,
        payload.instructions,
        payload.prep_time,
        payload.cook_time,
        payload.servings,
        payload.difficulty_level,
        payload.updated_by,
        payload.updated_at,
        recipe_uuid
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "Successfully update recipe").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update recipe").into_response(),
    }
}

pub async fn delete_recipe(
    State(state): State<AppState>,
    Path(recipe_uuid): Path<Uuid>,
) -> impl IntoResponse {
    let result = query!(
        r#"
        UPDATE recipes
        SET
            deleted_by = $1,
            deleted_at = $2
        WHERE uuid = $3
        "#,
        "system".to_string(),
        Utc::now(),
        recipe_uuid,
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "Successfully delete recipe").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete recipe").into_response(),
    }
}
