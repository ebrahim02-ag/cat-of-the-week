use axum::{routing::get, Router, extract::Path};
use std::{net::SocketAddr, time::Duration};
use window::{get_window, Window};
use sqlx::{Pool, postgres::PgPoolOptions, postgres::PgPool};

#[derive(Debug)]
struct User {
    name: String, 
    email: String
}


async fn root_handler() -> &'static str {
    let w = get_window(None);
    tokio::time::sleep(Duration::from_secs(5)).await;

    match w {
        Window::Upload => "Upload your photo",
        Window::Vote => "Vote for the cat",
        Window::Winner => "And the winner is..."
    }
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>  {
    let app = Router::new()
        .route("/", get(root_handler));

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    let database_url = "postgres://dboperator:operatorpass123@localhost:5243/postgres";
    let pool = PgPool::connect(database_url).await?;

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await?;

    println!("Query result: {}", row.0);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
