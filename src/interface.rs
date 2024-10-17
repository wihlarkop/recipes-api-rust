use crate::entities::Recipe;
use crate::error::CustomError;
use uuid::Uuid;

pub trait RecipeRepositories: Send + Sync {
    async fn get_recipes(&self) -> Result<Vec<Recipe>, sqlx::Error>;
    async fn get_recipe(&self, recipe_uuid: Uuid) -> Result<Recipe, sqlx::Error>;
    async fn create_recipe(&self, recipe: Recipe) -> Result<(), CustomError>;

    async fn update_recipe(&self, recipe: Recipe) -> Result<(), CustomError>;
    async fn delete_recipe(&self, recipe_uuid: Uuid) -> Result<(), CustomError>;
}
