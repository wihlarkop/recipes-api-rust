use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

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
    pub created_by: String,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub created_at: DateTime<Utc>,
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

    #[validate(custom(function = "validate_difficulty"))]
    pub difficulty_level: Option<String>,

    #[serde(skip_deserializing)]
    pub created_by: String,
    #[serde(skip_deserializing)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecipe {
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

fn validate_difficulty(difficulty: &String) -> Result<(), ValidationError> {
    let valid_levels = vec!["easy", "medium", "hard"];
    if !valid_levels.contains(&difficulty.to_lowercase().as_str()) {
        return Err(ValidationError::new("invalid_difficulty"));
    }
    Ok(())
}
