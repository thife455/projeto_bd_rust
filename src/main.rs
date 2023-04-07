mod api;
mod model;
mod repositories;

use std::env;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::user::*;
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

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(get_user_controller)
            .wrap(logger)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
