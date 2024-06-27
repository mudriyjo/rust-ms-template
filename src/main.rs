mod config;

use axum::{
    routing::get,
    Router
};
use std::sync::Arc;

use config::config::get_config;

#[tokio::main]
async fn main() {
    let config = get_config("dev");
    let server_address = config.server_address.clone();
    
    println!("{:?}", config);

    let app = Router::new()
        .route("/", get(|| async {"Hello world!"}))
        .with_state(Arc::new(config));
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
