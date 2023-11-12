mod router;

use std::{env, net::SocketAddr, path::Path};

use axum::Router;

fn cwd() -> String {
    return env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3000;
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    let file_path = Path::join(Path::new(&cwd()), "/db/todo.db");
    let app = Router::new().merge(router::get_router());

    println!("Opening server in {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
