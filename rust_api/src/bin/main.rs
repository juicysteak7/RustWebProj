use rust_api::{ Application, DataBase, Applications };
use serde::Serialize;
use tokio;
use tokio::net::TcpListener;
use axum::{ extract::Json, routing::put, routing::get, Router };
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
    .route("/api/update_application", put(update_application))
    .route("/api/delete_application", put(delete_application))
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

async fn update_application(Json(payload):Json<(Application, String)>) -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let result = db.update_application(payload.0, payload.1).await.unwrap();

    #[derive(Serialize)]
    struct ResponseMessage {
        applications: Applications
    }

    let mut applications = Applications::new();

    if let Some(app) = result {
        applications.add(app);
    }

    return Json(ResponseMessage {
        applications
    })
}

async fn delete_application(Json(payload):Json<Application>) -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let result = db.delete_application(payload).await.unwrap();

    #[derive(Serialize)]
    struct ResponseMessage {
        applications: Applications
    }

    let mut applications = Applications::new();

    if let Some(app) = result {
        applications.add(app);
    }

    return Json(ResponseMessage {
        applications
    })
}


async fn get_all_applications() -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let applications = db.get_all_applications().await;

    #[derive(Serialize)]
    struct ResponseMessage {
        applications: Applications,
    }
    match applications {
        Ok(apps) => {
            println!("Applications: {:?}", apps);

            Json(ResponseMessage {
                applications: apps,
            })
        }
        Err(e) => {
            eprintln!("{}",e);


            Json(ResponseMessage {
                applications: Applications::new(),
            })
        }
    }
}