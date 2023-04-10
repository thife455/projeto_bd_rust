use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct GymProductQuery1 {
    pub gym_name: String,
    pub product_name: String,
    pub price: i32,
}

