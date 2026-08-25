use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
};
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl, insert_into};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::establish_connection;

pub mod db;
pub mod models;
pub mod schema;

#[derive(Deserialize)]
struct CreateCityPayload {
    city_name: String,
}

// static AUTO_INC: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() {
    println!("Starting Server");

    // let conn = establish_connection();

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

async fn get_city(Path(city_id): Path<i32>) -> Result<Json<Value>, StatusCode> {
    let conn = &mut establish_connection();

    use schema::cities::dsl::*;

    let result = cities.filter(id.eq(city_id)).first::<models::City>(conn);

    match result {
        Ok(city) => Ok(Json(json!({
            "id": city.id,
            "name": city.name
        }))),
        Err(error) => {
            eprintln!("Error: {error}");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn create_city(Json(payload): Json<CreateCityPayload>) -> Result<Json<Value>, StatusCode> {
    if payload.city_name.len() > 8 {
        Err(StatusCode::BAD_REQUEST)
    } else {
        let conn = &mut establish_connection();

        use schema::cities::dsl::*;

        let inserted = insert_into(cities)
            .values(name.eq(payload.city_name))
            .returning(id)
            .get_result::<i32>(conn);

        match inserted {
            Ok(inserted_id) => Ok(Json(json!({
                "id": inserted_id
            }))),
            Err(error) => {
                eprintln!("Error: {error}");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}
