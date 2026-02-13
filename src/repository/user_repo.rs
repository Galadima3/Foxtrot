use sqlx::{Pool, Sqlite};
use crate::models::Person;

pub async fn find_all(
    pool: &Pool<Sqlite>,
) -> Result<Vec<Person>, sqlx::Error> {
    sqlx::query_as::<_, Person>("SELECT id, name FROM users")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(
    pool: &Pool<Sqlite>,
    id: i32,
) -> Result<Person, sqlx::Error> {
    sqlx::query_as::<_, Person>(
        "SELECT id, name FROM users WHERE id = ?1"
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn insert(
    pool: &Pool<Sqlite>,
    name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name) VALUES (?1)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(
    pool: &Pool<Sqlite>,
    id: i32,
    name: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE users SET name = ?1 WHERE id = ?2"
    )
    .bind(name)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete(
    pool: &Pool<Sqlite>,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM users WHERE id = ?1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
