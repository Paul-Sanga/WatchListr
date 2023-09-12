use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    // LOAD ENVIRONMENT VARIABLES
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to database");
            pool
        } 
        Err(err) => {
            println!("Failed to establish connection: {:?}", err);
            std::process::exit(1);
        }
    };

    // CREATE HEALTHCHECKER ROUTE
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    println!("Server is listening...");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust | Axum API | MySQL";
    // DEFINE A STRUCT FOR RESPONSE MESSAGE
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}
