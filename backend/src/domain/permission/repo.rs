use serde::{Deserialize, Serialize};
use crate::domain::error::RepoError;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission{
    pub id: i32,
    pub resource: String,
    pub action: Vec<Action>,
    pub description: String
}

#[async_trait::async_trait]
pub trait PermissionRepo: Send + Sync {
    async fn get(&self) -> Result<Vec<Permission>, RepoError>;
    async fn get_permissions_by_role_id(&self, role_id: i32)->Result<Vec<Permission>, RepoError>;
    async fn get_actions_by_ids(&self, ids: Vec<i32>)-> Result<Vec<Action>, RepoError>;
}


use std::str::FromStr;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type{
    CREATE,
    UPDATE,
    DELETE,
    READ,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::CREATE => "CREATE".into(),
            Type::UPDATE => "UPDATE".into(),
            Type::DELETE => "DELETE".into(),
            Type::READ => "READ".into(),
        }
    }
}
impl FromStr for Type {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATE" =>Ok(Type::CREATE),
            "UPDATE" =>Ok(Type::UPDATE),
            "DELETE" =>Ok(Type::DELETE),
            "READ" =>Ok(Type::READ),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action{
    pub id: i32,
    pub key: Type,
    pub description: String
}
