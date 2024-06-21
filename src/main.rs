mod config;

use axum::{
    routing::get,
    Router
};
use clap::Parser;
use config::config::{get_config};

#[tokio::main]
async fn main() {
    let config = get_config();
    println!("{:?}", config);
    let app = Router::new().route("/", get(|| async {"Hello world!"}));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
