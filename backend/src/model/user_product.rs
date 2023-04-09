use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct UserProduct {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserProduct {
    pub product_id: Uuid,
    pub user_id: Uuid,
}
