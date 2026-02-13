mod db;
mod handlers;
mod models;
mod error;
mod repository;
mod service;


use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{db::init_db};
use crate::handlers::user_handlers::{
    list_users, create_user, get_user, delete_user, update_user,
};
use sqlx::{Pool, Sqlite};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = init_db().await?;
    let app = app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

fn app(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/", get(|| async { "hello" }))
        .route("/list", get(list_users))
        .route("/add_person", post(create_user))
        .route("/person/{id}", get(get_user))
        .route("/remove_person/{id}", delete(delete_user))
        .route("/update_person/{id}", put(update_user))
        .with_state(pool)
}
