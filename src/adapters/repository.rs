use uuid::Uuid;
use sqlx::{Pool, Postgres};

use crate::models::user::User;
use crate::adapters::database::Config;

pub struct Repository {
    conn: Pool<Postgres>,
}

impl Repository {
    pub async fn new() -> Repository {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&Config::new().to_dsn())
            .await.unwrap();

        Repository {conn: pool}
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.conn)
            .await;
        result
    }

    pub async fn list_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.conn)
            .await;
        users
    }
}
