use crate::model::user::{CreateUser, DepositParams};
use crate::model::wallet::CreateWallet;
use crate::repositories::wallet::{find_wallet_by_user_id, update_balance};
use crate::repositories::{user::*, wallet::create_wallet};
use crate::AppState;
use actix_web::{
    delete, get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/user")]
pub async fn get_user_controller(state: Data<AppState>) -> impl Responder {
    let user_response = get_users(state).await;
    match user_response {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[post("/user")]
pub async fn create_user_controller(
    state: Data<AppState>,
    body: web::Json<CreateUser>,
) -> impl Responder {
    let params = body.into_inner();
    let response = create_user(state.clone(), params).await;
    match response {
        Ok(user) => match create_wallet(state, CreateWallet { user_id: user.id }).await {
            Ok(_wallet) => HttpResponse::Ok().json(user),
            Err(_e) => HttpResponse::InternalServerError().json("Error creating wallet"),
        },
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}

#[post("/user/{id}/deposit")]
pub async fn deposit_user(
    info: web::Path<(uuid::Uuid,)>,
    state: Data<AppState>,
    body: web::Json<DepositParams>,
) -> impl Responder {
    let user_id = info.0;

    let wallet = find_wallet_by_user_id(state.clone(), user_id)
        .await
        .unwrap();

    let new_balance = wallet.balance + body.into_inner().amount;

    let query_result = update_balance(state, wallet.id, new_balance).await;

    match query_result {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}

#[get("/user/{id}")]
pub async fn get_user_id_controller(
    info: web::Path<(uuid::Uuid,)>,
    state: Data<AppState>,
) -> impl Responder {
    match get_user_by_id(state, info.0).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Error in query"),
    }
}

#[delete("/user")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete_user")
}
