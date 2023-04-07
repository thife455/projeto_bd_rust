use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
