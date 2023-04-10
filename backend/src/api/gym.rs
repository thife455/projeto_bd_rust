use crate::repositories::gym::*;
use crate::repositories::user::list_user_by_gym;
use crate::AppState;
use crate::{model::gym::*, repositories::product::find_products_by_gym_id};
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

pub fn gym_scope() -> actix_web::Scope {
    web::scope("/gym")
        .service(get_gyms_controller)
        .service(get_gyms_city_controller)
        .service(get_gyms_users_controller)
        .service(create_gym_controller)
        .service(get_gym_products)
        .service(get_gym_by_id_controller)
}

#[get("")]
async fn get_gyms_controller(state: Data<AppState>) -> impl Responder {
    let gym_response = get_gyms(state).await;
    match gym_response {
        Ok(gyms) => HttpResponse::Ok().json(gyms),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[get("/{city}/city")]
async fn get_gyms_city_controller(
    info: web::Path<(String,)>,
    state: Data<AppState>,
) -> impl Responder {
    let city = info.0.clone();
    let gym_response = find_gym_by_city(state, city).await;
    match gym_response {
        Ok(gyms) => HttpResponse::Ok().json(gyms),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[get("/{id}/users")]
async fn get_gyms_users_controller(
    info: web::Path<(uuid::Uuid,)>,
    state: Data<AppState>,
) -> impl Responder {
    let gym_response = list_user_by_gym(state, info.0).await;
    match gym_response {
        Ok(gyms) => HttpResponse::Ok().json(gyms),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[post("")]
async fn create_gym_controller(
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

#[get("/{id}")]
async fn get_gym_by_id_controller(
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

#[get("/{id}/products")]
async fn get_gym_products(info: web::Path<(uuid::Uuid,)>, state: Data<AppState>) -> impl Responder {
    let gym_id = info.0;
    let products = find_products_by_gym_id(state, gym_id).await;

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().json("Error in query"),
    }
}
