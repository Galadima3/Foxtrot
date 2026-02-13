use sqlx::{Executor, Pool, Sqlite};


pub async fn init_db() -> anyhow::Result<Pool<Sqlite>> {
    let option = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("test.db")
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(option).await?;

    pool.execute(
        r#"
        CREATE TABLE if not exists users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE )
        "#,
    )
    .await?;
    

    Ok(pool)
}
