use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Gym {
    pub id: Uuid,
    pub name: String,
    pub address: String,
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct CreateGym {
    pub name: String,
    pub address: String,
}
