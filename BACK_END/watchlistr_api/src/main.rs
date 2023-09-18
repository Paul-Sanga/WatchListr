use axum::{
    extract::{self, Path},
    http::StatusCode,
    routing::{delete, get, post},
    Extension, Json, Router,
};

use dotenvy::dotenv;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("Failed to create Connection Pool! URL: {}", url));

    sqlx::migrate!("./migrations").run(&pool).await?;

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    // Validate that server is running
    println!("\nServer is listening on {}", addr);

    // Setup HTTP Server
    axum::Server::bind(&addr)
        .serve(app().layer(Extension(pool)).into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Show Model
#[derive(Serialize, Deserialize)]
pub struct Show {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub poster_url: String,
}

// Route configuration
#[allow(dead_code)]
fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/show", post(add_show))
        .route("/shows", get(get_shows))
        .route("/show/:id", delete(delete_show))
}

// Health checker
async fn handler() -> &'static str {
    "Connection established!"
}

// Get shows
async fn get_shows(state: Extension<Pool<Postgres>>) -> Json<Vec<Show>> {
    let Extension(pool) = state;

    let records = sqlx::query!("SELECT * FROM shows")
        .fetch_all(&pool)
        .await
        .expect("Failed to get shows...");

    let records = records
        .iter()
        .map(|r| Show {
            id: Some(r.id),
            name: r.name.to_string(),
            description: r.description.to_string(),
            poster_url: r.poster_url.to_string(),
        })
        .collect();

    Json(records)
}

// Add show | handler
pub async fn add_show(
    state: Extension<Pool<Postgres>>,
    extract::Json(show): extract::Json<Show>,
) -> Json<Show> {
    let Extension(pool) = state;

    let row = sqlx::query!(
        "INSERT INTO shows (name, description, poster_url) VALUES ($1, $2, $3) RETURNING id, name, description, poster_url",
        show.name,
        show.description,
        show.poster_url,
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to add show...");

    Json(Show {
        id: Some(row.id),
        name: row.name,
        description: row.description,
        poster_url: row.poster_url,
    })
}

// Delete show | handler
pub async fn delete_show(state: Extension<Pool<Postgres>>, Path(show_id): Path<i32>) -> StatusCode {
    let Extension(pool) = state;

    sqlx::query!("DELETE FROM shows WHERE id = $1", show_id)
        .execute(&pool)
        .await
        .expect("Failed to delete show...");

    StatusCode::NO_CONTENT
}
