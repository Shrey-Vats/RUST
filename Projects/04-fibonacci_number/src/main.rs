use axum::{response::Json, routing::{get, post}, Router};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // building application with a single route
    let app =  Router::new()
        .route("/get", post(get_fibonacci_number(11)))
        .route("/", get(call_the_backend()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let value = get_fibonacci_number(10);

    println!("The array of nums is {:?}", value);
}

fn call_the_backend() -> Json<Value> {
    Json(json!("Welcome to rust mini backend project 1"))
}

fn get_fibonacci_number(value: usize) -> Json<Value> {
    let mut num: Vec<i32> = vec![0, 1];
    for i in 2..=value - 1 {
        num.push(num[i - 1] + num[i - 2]);
    }
    Json(json!(num[value - 1]))
}
