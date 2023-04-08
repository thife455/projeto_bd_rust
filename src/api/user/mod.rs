use crate::model::user::CreateUser;
use crate::model::wallet::CreateWallet;
use crate::repositories::{user::*, wallet::create_wallet};
use crate::AppState;
use actix_web::{
    delete, get, post, put,
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

#[put("/user")]
pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("update_user")
}

#[delete("/user")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete_user")
}
