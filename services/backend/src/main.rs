use axum::{routing::get, Router, extract::Path};
use std::net::SocketAddr;

async fn root_handler() -> &'static str {
    "Hellooooooooooooooooo"
}

async fn ping_handler() -> &'static str {
    "pong"
}


// Handler to greet a user by name from the path
async fn greet_handler(Path(name): Path<String>) -> String {
    // Path(name) extracts the parameter named 'name' from the URL
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/ping", get(ping_handler))
        .route("/greet/{name}", get(greet_handler));

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
