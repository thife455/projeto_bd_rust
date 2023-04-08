use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Wallet {
    pub id: Uuid,
    pub balance: i32,
    pub user_id: Uuid,
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct CreateWallet {
    pub user_id: Uuid,
}
