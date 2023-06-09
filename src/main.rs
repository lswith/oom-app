//! An app which will try and consume large amount of memory on startup.
//! This app is used to test our resource usage limits.
use std::{
    env,
    sync::{Arc, Mutex},
};

use axum::{extract::State, routing::get, Router};
use tokio::task;

#[tokio::main]
async fn main() {
    let data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

    let data_clone = data.clone();

    task::spawn(async move {
        loop {
            {
                let mut newdata: Vec<u8> = vec![u8::MAX; 100 * MEGABYTES];
                let mut data = data_clone.lock().unwrap();
                data.append(&mut newdata);
            }
            println!("Adding more data to the vector");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(data.clone());

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    println!("Starting Server on {port}");
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const KILOBYTES: usize = 1024;
const MEGABYTES: usize = 1024 * KILOBYTES;

#[allow(clippy::unused_async)]
async fn handler(State(state): State<Arc<Mutex<Vec<u8>>>>) -> String {
    {
        let mut newdata: Vec<u8> = vec![u8::MAX; 100 * MEGABYTES];
        let mut data = state.lock().unwrap();
        data.append(&mut newdata);
    }
    let s = format!("Added {} elements to the shared vector", usize::MAX);
    println!("{s}");
    s
}
