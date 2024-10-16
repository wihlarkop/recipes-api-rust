mod config;
mod database;
mod entities;
mod error;
mod handlers;
mod helper;
mod interface;
mod repositories;
mod state;
mod validation;

use crate::config::Config;
use crate::database::database_init;
use crate::handlers::{create_recipe, delete_recipe, get_all_recipes, get_recipe, update_recipe};
use crate::state::AppState;
use axum::{routing::get, Router};
use envconfig::Envconfig;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed get .env");
    let config = Config::init_from_env().unwrap();

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let pool = database_init(config.database_url).await;

    let state = AppState::new(pool.unwrap()).await;

    let app = Router::new()
        .route("/recipe", get(get_all_recipes).post(create_recipe))
        .route(
            "/recipe/:recipe_uuid",
            get(get_recipe).put(update_recipe).delete(delete_recipe),
        )
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
