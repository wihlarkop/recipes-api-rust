use crate::entities::Recipe;
use crate::error::CustomError;
use crate::interface::RecipeRepositories;
use crate::state::AppState;
use chrono::Utc;
use sqlx::{query, query_as};
use uuid::Uuid;

impl RecipeRepositories for AppState {
    async fn get_recipes(&self) -> Result<Vec<Recipe>, sqlx::Error> {
        query_as!(Recipe, r#"SELECT * from recipes where deleted_at is null"#)
            .fetch_all(&self.db)
            .await
    }
    async fn get_recipe(&self, recipe_uuid: Uuid) -> Result<Recipe, sqlx::Error> {
        query_as!(
            Recipe,
            r#"SELECT * FROM recipes WHERE uuid = $1"#,
            recipe_uuid
        )
        .fetch_one(&self.db)
        .await
    }

    async fn create_recipe(&self, recipe: Recipe) -> Result<(), CustomError> {
        let result = query!(
        r#"
        INSERT INTO recipes (uuid, name, description, ingredients, instructions, prep_time, cook_time, servings, difficulty_level, created_by, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        recipe.uuid,
        recipe.name,
        recipe.description,
        recipe.ingredients,
        recipe.instructions,
        recipe.prep_time,
        recipe.cook_time,
        recipe.servings,
        recipe.difficulty_level,
        recipe.created_by,
        recipe.created_at
    ).execute(&self.db).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::FailedInsertRecipe),
        }
    }

    async fn update_recipe(&self, recipe: Recipe) -> Result<(), CustomError> {
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
            recipe.name,
            recipe.description,
            recipe.ingredients,
            recipe.instructions,
            recipe.prep_time,
            recipe.cook_time,
            recipe.servings,
            recipe.difficulty_level,
            recipe.updated_by,
            recipe.updated_at,
            recipe.uuid
        )
        .execute(&self.db)
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::FailedUpdateRecipe),
        }
    }

    async fn delete_recipe(&self, recipe_uuid: Uuid) -> Result<(), CustomError> {
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
        .execute(&self.db)
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::FailedDeleteRecipe),
        }
    }
}
