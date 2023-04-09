use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// pub async fn create_pool() -> Pool<Postgres> {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     std::env::set_var(
//         "DATABASE_URL",
//         "postgres://thife455:thife455@localhost/gym_test_dev",
//     );
//     PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await
//         .expect("Error building a connection pool")
// }
