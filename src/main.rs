use axum::{
    response::Json,
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod models;
use models::user::User;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handler() -> Json<Vec<User>> {
    let dsn = "postgres://test:test@172.56.57.100:5432/revolcane";

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(dsn)
        .await.unwrap();

    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await.unwrap();

    Json(users)
}
