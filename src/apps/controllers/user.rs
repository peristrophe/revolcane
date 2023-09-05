use uuid::Uuid;
use crate::adapters::repository::Repository;
use crate::models::user::User;

pub struct UserController {
    repository: Repository
}

impl UserController {
    pub async fn new() -> UserController {
        let repo = Repository::new().await;
        let uc = UserController {repository: repo};
        uc
    }

    pub async fn user_detail_view(&self, user_id: Uuid) -> User {
        self.repository.get_user(user_id).await.unwrap()
    }
}