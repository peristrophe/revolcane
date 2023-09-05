use axum::{
    extract::Path,
    routing::get,
    Router,
    response::Json,
};
use uuid::Uuid;

use crate::apps::controllers::user::UserController;
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

async fn get_user_handler(Path(user_id): Path<Uuid>) -> Json<User> {
    let controller = UserController::new().await;
    let user = controller.user_detail_view(user_id).await;
    Json(user)
}