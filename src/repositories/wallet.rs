use crate::{model::wallet::*, AppState};
use actix_web::web::Data;
use sqlx::Error;

pub async fn create_wallet(
    state: Data<AppState>,
    wallet_data: CreateWallet,
) -> Result<Wallet, Error> {
    let id = uuid::Uuid::new_v4();
    let balance = 0;
    let CreateWallet { user_id } = wallet_data;

    let response = sqlx::query_as!(
        Wallet,
        "INSERT INTO wallets (id, balance, user_id) VALUES ($1, $2, $3) RETURNING id, balance, user_id",
        id,
        balance,
        user_id
    )
    .fetch_one(&state.db)
    .await;

    match response {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    }
}
