use crate::{model::user::User, AppState};
use crate::repositories::user::*;
use actix_web::HttpRequest;
use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/user")]
pub async fn get_user_controller(state: Data<AppState>) -> impl Responder {
    let user_response = get_users(state).await;
    match user_response {
        Ok(users) => return HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[post("/user")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("create_user")
}

#[put("/user")]
pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("update_user")
}

#[delete("/user")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete_user")
}
