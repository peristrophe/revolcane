use axum::{
    response::Json,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use once_cell::sync::Lazy;

mod models;
use models::user::User;

mod repositories;
use repositories::database::Config;

static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    postgres_host: std::env::var("DB_HOST").unwrap(),
    postgres_port: std::env::var("DB_PORT").unwrap(),
    postgres_user: std::env::var("DB_USER").unwrap(),
    postgres_password: std::env::var("DB_PASSWORD").unwrap(),
    postgres_database: std::env::var("DB_NAME").unwrap(),
});

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handler() -> Json<Vec<User>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&CONFIG.to_dsn())
        .await.unwrap();

    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await.unwrap();

    Json(users)
}
