use serde::{Deserialize, Serialize};

use crate::domain::{error::RepoError, role::repo::Role};
use common_model::user::{CreateUserRequest,FilterUserRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  UserIdentity {
    pub email: String,
    pub user_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub employee_id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    //pub roles: Vec<Role>,
}

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn get(&self, filter: FilterUserRequest) -> Result<Vec<User>, RepoError>;
    async fn get_by_id(&self, id: i32) -> Result<User, RepoError>;
    async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, RepoError>;
    async fn create(&self, username: String, email: String, password_hash: String) -> Result<User, RepoError>;
    async fn update(&self, id: i32, user: User) -> Result<User, RepoError>;
    async fn delete_by_id(&self, id: i32) -> Result<i32, RepoError>;
    async fn delete_list_ids(&self, id: Vec<i32>) -> Result<Vec<i32>, RepoError>;
}

