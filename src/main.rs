mod models;
mod router;
mod utils;
use std::{env, net::SocketAddr, path::Path};

use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3000;
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    let app = Router::new().merge(router::get_router().await);

    println!("Opening server in {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
