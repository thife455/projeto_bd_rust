use crate::repositories::gym::*;
use crate::AppState;
use crate::{model::gym::*, repositories::product::find_products_by_gym_id};
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/user/{name}")]
pub async fn get_user_by_name_controller(
    info: web::Path<(uuid::Uuid,)>,
    state: Data<AppState>,
) -> impl Responder {
    let id = info.0;

    let user = find_user_by_name(state, name).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Error in query"),
    }
}
