use axum::{
    Json, Router, http::StatusCode, routing::get,
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    println!("Starting Server");
    
    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/data", get(data))
        .route("/bad", get(bad_request))
    ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
