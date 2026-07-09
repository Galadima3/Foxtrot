use sqlx::{Pool, Sqlite};

pub async fn init_db() -> anyhow::Result<Pool<Sqlite>> {
    let option = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("test.db")
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(option).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
