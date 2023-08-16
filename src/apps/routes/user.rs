use axum::{
    extract::Path,
    routing::get,
    Router,
    response::Json,
};
use uuid::Uuid;

use crate::adapters::repository::Repository;
use crate::models::user::User;

pub fn user_routes() -> Router {
    let r = Router::new();
    let r = r.route("/", get(list_users_handler));
    let r = r.route("/:id", get(get_user_handler));
    r
}

async fn list_users_handler() -> Json<Vec<User>> {
    let repository = Repository::new().await;
    let users = repository.list_users().await.unwrap();
    Json(users)
}

async fn get_user_handler(Path(params): Path<Uuid>) -> Json<User> {
    let user_id = params;
    let repository = Repository::new().await;
    let user = repository.get_user(user_id).await.unwrap();
    Json(user)
}