use sqlx::{Error, PgPool, Pool, Postgres};

pub async fn database_init() -> Result<Pool<Postgres>, Error> {
    PgPool::connect("postgres://postgres:admin@localhost/recipes").await
}