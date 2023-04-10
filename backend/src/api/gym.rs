use crate::repositories::gym::*;
use crate::AppState;
use crate::{model::gym::*, repositories::product::find_products_by_gym_id};
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/gym")]
pub async fn get_gyms_controller(state: Data<AppState>) -> impl Responder {
    let gym_response = get_gyms(state).await;
    match gym_response {
        Ok(gyms) => HttpResponse::Ok().json(gyms),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[post("/gym")]
pub async fn create_gym_controller(
    state: Data<AppState>,
    body: web::Json<CreateGym>,
) -> impl Responder {
    let params = body.into_inner();
    let response = create_gym(&state, params).await;
    match response {
        Ok(gym) => HttpResponse::Ok().json(gym),
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}

#[get("/gym/{id}")]
pub async fn get_gym_by_id_controller(
    info: web::Path<(uuid::Uuid,)>,
    state: Data<AppState>,
) -> impl Responder {
    let id = info.0;

    let gym = find_gym_by_id(state, id).await;

    match gym {
        Ok(gym) => HttpResponse::Ok().json(gym),
        Err(_) => HttpResponse::InternalServerError().json("Error in query"),
    }
}

#[get("/gym/{id}/products")]
pub async fn get_gym_products(
    info: web::Path<(uuid::Uuid,)>,
    state: Data<AppState>,
) -> impl Responder {
    let gym_id = info.0;
    let products = find_products_by_gym_id(state, gym_id).await;

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().json("Error in query"),
    }
}
