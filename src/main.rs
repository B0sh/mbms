use axum::{
    Json, Router, extract::Path, http::StatusCode, routing::{get, post},
};
use serde_json::{Value, json};

#[derive(Deserialize)]
struct CreateCityPayload {
    city_name: String
}

static mut GLOBAL_ID: i32 = 0;

#[tokio::main]
async fn main() {
    println!("Starting Server");
    
    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/data", get(data))
        .route("/bad", get(bad_request))
        .route("/city", post({
            move |body| create_city(body)
        }))
        .route("/city/{id}", get({
            move |path| get_city(path)
        }))
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

async fn get_city(Path(city_id): Path<String>) -> Json<Value> {
    Json(json!({
        "id": city_id,
        "name": format!("City #{}", city_id)
    }))
}

async fn create_city(Json(payload): Json<CreateCityPayload>) -> Result<Json<Value>, StatusCode> {
    if payload.city_name.len() > 8 {
        Err(StatusCode::BAD_REQUEST)
    }
    else {
        unsafe {
            let id = GLOBAL_ID;
            GLOBAL_ID += 1;
        }
        Ok(Json(json!({
            "id": id,
            "name": format!("City #{}", id)
        })))
    }
}
