use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) db: Pool<Postgres>,
}

impl AppState {
    pub async fn new(db: Pool<Postgres>) -> AppState {
        Self { db: db.clone() }
    }
}
