
use serde::{Deserialize, Serialize};

use crate::domain::{error::RepoError};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role{
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[async_trait::async_trait]
pub trait RoleRepo: Send + Sync {
    async fn get(&self) -> Result<Vec<Role>, RepoError>;
    async fn get_by_id(&self, id: i32) -> Result<Role, RepoError>;
    async fn create(&self, name: String, description: String) -> Result<Role, RepoError>;
    async fn update(&self, id: i32, name: String, description: String) -> Result<Role, RepoError>;
    async fn delete_by_id(&self, id: i32) -> Result<i32, RepoError>;
    async fn get_roles_by_user_id(&self, user_id: i32) -> Result<Vec<Role>, RepoError>;
}