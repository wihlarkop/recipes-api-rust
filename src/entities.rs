use crate::validation;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ingredients: String,
    pub instructions: String,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub servings: Option<i32>,
    pub difficulty_level: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct InsertRecipe {
    #[serde(skip_deserializing)]
    pub uuid: Uuid,
    #[validate(length(min = 1, message = "name cannot empty"))]
    pub name: String,
    pub description: Option<String>,
    pub ingredients: String,
    pub instructions: String,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub servings: Option<i32>,

    #[validate(custom(function = "validation::validate_difficulty"))]
    pub difficulty_level: Option<String>,

    #[serde(skip_deserializing)]
    pub created_by: String,
    #[serde(skip_deserializing)]
    pub created_at: DateTime<Utc>,
}

impl From<InsertRecipe> for Recipe {
    fn from(insert_recipe: InsertRecipe) -> Self {
        Recipe {
            uuid: Uuid::new_v4(), // Buat UUID baru untuk recipe
            name: insert_recipe.name,
            description: insert_recipe.description,
            ingredients: insert_recipe.ingredients,
            instructions: insert_recipe.instructions,
            prep_time: insert_recipe.prep_time,
            cook_time: insert_recipe.cook_time,
            servings: insert_recipe.servings,
            difficulty_level: insert_recipe.difficulty_level,
            created_by: Some(insert_recipe.created_by),
            updated_by: None,
            deleted_by: None,
            created_at: Some(Utc::now()),
            updated_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecipe {
    #[serde(skip_deserializing)]
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ingredients: String,
    pub instructions: String,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub servings: Option<i32>,
    pub difficulty_level: Option<String>,
    #[serde(skip_deserializing)]
    pub updated_by: String,
    #[serde(skip_deserializing)]
    pub updated_at: DateTime<Utc>,
}

impl From<UpdateRecipe> for Recipe {
    fn from(update_recipe: UpdateRecipe) -> Self {
        Recipe {
            uuid: update_recipe.uuid,
            name: update_recipe.name,
            description: update_recipe.description,
            ingredients: update_recipe.ingredients,
            instructions: update_recipe.instructions,
            prep_time: update_recipe.prep_time,
            cook_time: update_recipe.cook_time,
            servings: update_recipe.servings,
            difficulty_level: update_recipe.difficulty_level,
            created_by: None,
            updated_by: Some("system".to_string()),
            deleted_by: None,
            created_at: None,
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }
}
