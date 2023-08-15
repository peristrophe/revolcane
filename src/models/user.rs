use uuid::Uuid;
use chrono::prelude::{DateTime, Utc};

use serde::Serialize;

#[derive(sqlx::Type, Serialize)]
#[repr(i32)]
pub enum UserStatus {
    Free = 0,
    Busy = 1,
    Sick = 2,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub salt: Uuid,
    pub name: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
