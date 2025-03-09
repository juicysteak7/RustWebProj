use rust_api::Application;
use rust_api::DataBase;
use rust_api::Applications;
use serde::Serialize;
use tokio;
use tokio::net::TcpListener;
use axum::{
    extract::{Path, Json},
    response::IntoResponse,
    routing::put,
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let cors = CorsLayer::new()
    .allow_origin(Any) // Allow all origins (*)
    .allow_methods(Any) // Allow GET, PUT, DELETE, POST, etc.
    .allow_headers(Any); // Allow all headers

    let app = Router::new()
    .route("/api/add_application", put(add_application))
    .route("/api/get_all_applications", get(get_all_applications))
    .layer(cors);
    let addr:SocketAddr = "127.0.0.1:6969".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();

    println!("Server running on {}", addr);
    Ok(())
}

async fn add_application(Json(payload):Json<Application>) -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let result = db.create_application(payload).await.unwrap();

    #[derive(Serialize)]
    struct ResponseMessage {
        applications: Applications
    }

    let mut applications = Applications::new();

    match result { Some(result) => {
        applications.add(result);
        return Json(ResponseMessage {
            applications
        })
    } 
    None => {
        return Json(ResponseMessage {
            applications
        })
    }}
}

async fn get_all_applications() -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let applications = db.get_all_applications().await.unwrap();

    println!("Applications: {:?}", applications);

    #[derive(Serialize)]
    struct ResponseMessage {
        applications: Applications,
    }
    Json(ResponseMessage {
        applications,
    })
}