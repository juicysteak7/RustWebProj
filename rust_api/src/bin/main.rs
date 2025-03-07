use rust_api::DataBase;
use serde::Serialize;
use tokio;
use tokio::net::TcpListener;
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let _db = DataBase::sign_in("root", "root").await?;

    let app = Router::new().route("/api/message", get(api_message)).layer(CorsLayer::new().allow_origin(Any));
    let addr:SocketAddr = "127.0.0.1:6969".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();

    println!("Server running on {}", addr);
    Ok(())
}

async fn api_message() -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;
    println!("Here");

    #[derive(Serialize)]
    struct ResponseMessage {
        message: String,
    }
    Json(ResponseMessage {
        message:"Message from the backend".to_string(),
    })
}