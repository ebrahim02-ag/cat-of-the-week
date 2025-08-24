use axum::{routing::get, Router, extract::Path};
use std::net::SocketAddr;
use window::{get_window, Window};

async fn root_handler() -> &'static str {
    let w = get_window(None);
    match w {
        Window::Upload => "Upload your photo",
        Window::Vote => "Vote for the cat",
        Window::Winner => "And the winner is..."
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler));


    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
