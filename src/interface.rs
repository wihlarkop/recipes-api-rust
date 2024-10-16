use crate::entities::Recipe;
use uuid::Uuid;

pub trait RecipeRepositories: Send + Sync {
    async fn get_recipe(&self, recipe_uuid: Uuid) -> Result<Recipe, sqlx::Error>;
}
