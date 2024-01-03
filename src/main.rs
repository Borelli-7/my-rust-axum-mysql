use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;

pub struct AppState{
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let database_url= std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set ");
    let pool= match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("Connected to MySQL database successfully !");
                pool
            }
            Err(err) => {
                println!("Failed to connect to MySQL database: {:?}", err);
                std::process::exit(1);
            }

        };

    let app= Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState{db: pool.clone()}));

    println!("Server Starting successfully !");
    let listener= tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await.unwrap();
    axum::serve(listener, app)
        .await.unwrap()
   
    
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust CRUD API Example with Axum Framework and MySQL";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}