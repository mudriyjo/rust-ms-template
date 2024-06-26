mod config;

use axum::{
    routing::get,
    Router
};

use config::config::get_config;

#[tokio::main]
async fn main() {
    let config = get_config("dev");
    println!("{:?}", config);
    let app = Router::new().route("/", get(|| async {"Hello world!"}));
    let listener = tokio::net::TcpListener::bind(config.server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
