use axum::{
    routing::{post, get},
    extract::State,
    http::StatusCode,
    Json,
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::routes::auth::{login, signup};
use tower_http::cors::CorsLayer;

mod routes;
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load environment variables from .env
    dotenv::dotenv().ok();
    let cors=CorsLayer::new()
        // Allow requests from your front-end origin.
        // For development, you can allow all origins using `any()`.
        .allow_origin(tower_http::cors::Any)
        // Allow common headers
        .allow_headers([axum::http::header::CONTENT_TYPE])
        // Allow the POST method
        .allow_methods([axum::http::Method::POST]);
    // Database pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let pool = PgPool::connect(&database_url).await?;

    // Build our application with routes
    let app = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/dashboard", get(routes::dashboard::dashboard))  // Protected route
        .with_state(pool).layer(cors);

    // Start server
    let addr = TcpListener::bind(("0.0.0.0", 3000)).await?;
    println!("Server running at http://{:?}", addr);
    axum::serve(addr,app).await?;
    Ok(())
}
