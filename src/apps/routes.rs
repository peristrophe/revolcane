pub mod user;
use user::user_routes;

pub fn create_app() -> axum::Router {
    let api_routes = axum::Router::new()
        .nest("/users", user_routes());

    axum::Router::new().nest("/api/v1", api_routes)
}
