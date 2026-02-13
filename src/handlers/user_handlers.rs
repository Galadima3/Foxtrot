use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{Pool, Sqlite};
use crate::{
    error::AppError,
    models::{UserRequest},
    service::user_service,
};

pub async fn list_users(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    match user_service::get_all(&pool).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users").into_response(),
    }
}

pub async fn get_user(State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>) -> impl IntoResponse {
    match user_service::get_one(&pool, id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(AppError::NotFound) => (StatusCode::NOT_FOUND, format!("User id {} not found", id)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch user").into_response(),
    }
}

pub async fn create_user(State(pool): State<Pool<Sqlite>>, Json(req): Json<UserRequest>) -> impl IntoResponse {
    match user_service::create(&pool, &req.name).await {
        Ok(_) => (StatusCode::CREATED, format!("User '{}' created", req.name)).into_response(),
        Err(AppError::Conflict) => (StatusCode::CONFLICT, format!("Username '{}' already exists", req.name)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}

pub async fn update_user(State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>, Json(req): Json<UserRequest>) -> impl IntoResponse {
    match user_service::update(&pool, id, &req.name).await {
        Ok(_) => (StatusCode::OK, format!("User id {} updated", id)).into_response(),
        Err(AppError::NotFound) => (StatusCode::ACCEPTED, format!("User id {} not found", id)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user").into_response(),
    }
}

pub async fn delete_user(State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>) -> impl IntoResponse {
    match user_service::delete(&pool, id).await {
        Ok(_) => (StatusCode::NO_CONTENT, format!("User id {} deleted", id)).into_response(),
        Err(AppError::NotFound) => (StatusCode::NOT_FOUND, format!("User id {} not found", id)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user").into_response(),
    }
}
