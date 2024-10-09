mod entities;
mod database;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router, routing::get};
use sqlx::{query_as, Pool, Postgres};
use crate::database::database_init;
use crate::entities::Recipe;

#[derive(Debug, Clone)]
struct AppState {
    db: Pool<Postgres>,
}

impl AppState {
    pub async fn new(db: Pool<Postgres>) -> AppState {
        Self {
            db: db.clone(),
        }
    }
}

#[tokio::main]
async fn main() {
    let pool = database_init().await;

    let state = AppState::new(pool.unwrap());

    let app = Router::new()
        .route("/recipes", get(get_all_recipes))
        .with_state(state.await);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_recipes(State(state): State<AppState>) -> impl IntoResponse {
    match query_as!(Recipe, r#"SELECT * from recipes"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(recipes) => (StatusCode::OK, Json(recipes)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch recipes").into_response(),
    }
}
