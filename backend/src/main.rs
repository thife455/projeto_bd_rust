mod api;
mod model;
mod repositories;
mod services;

use std::env;

use actix_cors::Cors;
use actix_web::web;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::product::{
    buy_product_controller, create_products_controller, get_product_gym_controller, products_scope,
};
use api::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var(
        "DATABASE_URL",
        "postgres://thife455:thife455@localhost/gym_test_dev",
    );
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    fn services() -> actix_web::Scope {
        web::scope("")
            .service(product::products_scope())
            .service(gym::gym_scope())
            .service(user::user_scope())
            .service(user_product::user_product_scope())
    }
    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(services())
            .service(create_products_controller)
            .service(buy_product_controller)
            .service(get_product_gym_controller)
            .wrap(logger)
            .wrap(cors)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
