use sqlx::{Pool, Sqlite};
use crate::{
    error::AppError,
    models::Person,
    repository::user_repo,
};

pub async fn get_all(
    pool: &Pool<Sqlite>,
) -> Result<Vec<Person>, AppError> {
    user_repo::find_all(pool)
        .await
        .map_err(|_| AppError::Database)
}

pub async fn get_one(
    pool: &Pool<Sqlite>,
    id: i32,
) -> Result<Person, AppError> {
    match user_repo::find_by_id(pool, id).await {
        Ok(user) => Ok(user),

        Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),

        Err(_) => Err(AppError::Database),
    }
}

pub async fn create(
    pool: &Pool<Sqlite>,
    name: &str,
) -> Result<(), AppError> {
    match user_repo::insert(pool, name).await {
        Ok(_) => Ok(()),

        Err(sqlx::Error::Database(db_err))
            if db_err.message().contains("UNIQUE") =>
        {
            Err(AppError::Conflict)
        }

        Err(_) => Err(AppError::Database),
    }
}

pub async fn update(
    pool: &Pool<Sqlite>,
    id: i32,
    name: &str,
) -> Result<(), AppError> {
    let affected = user_repo::update(pool, id, name)
        .await
        .map_err(|_| AppError::Database)?;

    if affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn delete(
    pool: &Pool<Sqlite>,
    id: i32,
) -> Result<(), AppError> {
    let affected = user_repo::delete(pool, id)
        .await
        .map_err(|_| AppError::Database)?;

    if affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
