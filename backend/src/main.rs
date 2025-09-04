mod db;
mod models;
mod routes;

use axum::{routing::{get, post}, Router};
use dotenv::dotenv;
use std::env;
use sqlx::PgPool;
use crate::routes::users::{get_user_handler,create_user};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (put it in .env or env vars)");

    // initialize DB
    let pool: PgPool = db::init_db(&database_url).await?;

    // build routes
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/:id",get(get_user_handler))
        .route("/contacts", post(routes::contact::create_contact_handler).get(routes::contact::get_all_contact_data_handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
