use crate::model::wallet::*;
use actix_web::web;
use sqlx::Error::{self, *};
use uuid::Uuid;

use crate::{
    repositories::wallet::{find_wallet_by_user_id, update_balance},
    AppState,
};

pub async fn debit(
    state: web::Data<AppState>,
    price: &i32,
    user_id: Uuid,
) -> Result<(), &'static str> {
    let wallet = match find_wallet_by_user_id(state.clone(), user_id).await {
        Ok(wallet) => Ok::<Wallet, &str>(wallet),
        Err(_) => Err::<Wallet, &str>("query_error"),
    }?;

    if &wallet.balance < price {
        return Err("no_balance");
    }

    let new_balance = &wallet.balance - price;

    let wallet = update_balance(state, wallet.id, new_balance).await;

    match wallet {
        Ok(_wallet) => Ok::<(), &'static str>(()),
        Err(_) => Err("no_update"),
    }
}
