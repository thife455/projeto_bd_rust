use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct GymProductQuery2 {
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
    pub sum_price: i32,
}
