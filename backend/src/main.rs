use axum::{
    routing::{post, get},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use crate::api::auth::{login, signup};
use tower_http::cors::CorsLayer;
use crate::api::dashboard::dashboard;
use crate::api::contact::{create_contact_handler,get_all_contact_data_handler};

mod models;
mod utils;
mod api;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let cors=CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_private_network(true)
        .allow_methods([axum::http::Method::POST]);
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        // .route("/signup", post(signup))
        // .route("/login", post(login))
        .route("/contacts",post(create_contact_handler))
        .route("/dashboard", get(dashboard))
        .with_state(pool).layer(cors);

    let addr = TcpListener::bind(("0.0.0.0", 3000)).await?;
    println!("Server running at http://{:?}", addr);
    axum::serve(addr,app).await?;
    Ok(())
}