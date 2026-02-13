use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// structs
#[derive(FromRow, Serialize)]
pub struct Person {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub name: String,
}
