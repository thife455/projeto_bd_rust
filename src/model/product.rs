use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: i32,
    pub gym_id: Uuid,
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: i32,
    pub gym_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct BuyProduct {
    pub user_id: Uuid,
}
