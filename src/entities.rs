use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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