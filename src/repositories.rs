use crate::entities::Recipe;
use crate::interface::RecipeRepositories;
use crate::state::AppState;
use sqlx::query_as;
use uuid::Uuid;

impl RecipeRepositories for AppState {
    async fn get_recipe(&self, recipe_uuid: Uuid) -> Result<Recipe, sqlx::Error> {
        query_as!(
            Recipe,
            r#"SELECT * FROM recipes WHERE uuid = $1"#,
            recipe_uuid
        )
            .fetch_one(&self.db).await
    }
}
