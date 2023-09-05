use std::net::SocketAddr;

mod apps;
mod models;
mod adapters;

#[tokio::main]
async fn main() {
    let app = apps::routes::create_app();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
