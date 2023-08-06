use axum::{
    response::Json,
    routing::get,
    Router,
};
use uuid::Uuid;
use chrono::{prelude::{DateTime, Utc}, TimeZone};

use std::net::SocketAddr;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

async fn handler() -> Json<User> {
    let datetime = Utc.with_ymd_and_hms(2023, 8, 6, 0, 0, 0).unwrap();
    let user = User {
        id: Uuid::new_v4(),
        name: String::from("hoge"),
        created_at: datetime,
        updated_at: datetime,
    };
    Json(user)
}
