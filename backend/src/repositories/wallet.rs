use crate::{model::wallet::*, AppState};
use actix_web::web::Data;
use sqlx::Error;
use uuid::Uuid;

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

pub async fn find_wallet_by_user_id(state: Data<AppState>, user_id: Uuid) -> Result<Wallet, Error> {
    let query_response =
        sqlx::query_as!(Wallet, "SELECT * FROM wallets WHERE user_id = $1", user_id)
            .fetch_one(&state.db)
            .await;

    match query_response {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    }
}

pub async fn update_balance(
    state: Data<AppState>,
    wallet_id: Uuid,
    new_balance: i32,
) -> Result<Wallet, Error> {
    let query_response = sqlx::query_as!(
        Wallet,
        "UPDATE wallets SET balance = $1 WHERE user_id = $2 RETURNING *",
        new_balance,
        wallet_id
    )
    .fetch_one(&state.db)
    .await;

    match query_response {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    }
}
