use sqlx::{Error, PgPool, Pool, Postgres};

pub async fn database_init(database_url: String) -> Result<Pool<Postgres>, Error> {
    PgPool::connect(&database_url).await
}
