use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Deserialize)]
struct CreateCityPayload {
    city_name: String,
}

static AUTO_INC: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() {
    println!("Starting Server");

    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/data", get(data))
        .route("/bad", get(bad_request))
        .route("/city", post(move |body| create_city(body)))
        .route("/city/{id}", get(move |path| get_city(path)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6655").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> String {
    "Hello, World!".to_string()
}

async fn bad_request() -> Result<String, StatusCode> {
    Err(StatusCode::BAD_REQUEST)
}

async fn data() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn get_city(Path(city_id): Path<String>) -> Json<Value> {
    Json(json!({
        "id": city_id,
        "name": format!("City #{}", city_id)
    }))
}

async fn create_city(Json(payload): Json<CreateCityPayload>) -> Result<Json<Value>, StatusCode> {
    if payload.city_name.len() > 8 {
        Err(StatusCode::BAD_REQUEST)
    } else {
        // https://doc.rust-lang.org/beta/std/sync/atomic/enum.Ordering.html
        let id = AUTO_INC.fetch_add(1, Ordering::SeqCst);
        println!("Creating City #{}", id);

        Ok(Json(json!({
            "id": id,
            "name": format!("City #{}", id)
        })))
    }
}
