use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{permission::repo::PermissionRepo, role::repo::RoleRepo, user::repo::UserRepo};



#[async_trait]
pub trait UserService:Sync + Send {
    
}

#[derive(Clone)]
pub struct UserServiceImpl{
    pub user_repo: Arc<dyn UserRepo>,
    pub role_repo: Arc<dyn RoleRepo>,
    pub permission_repo: Arc<dyn PermissionRepo>,
}

impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepo>,role_repo: Arc<dyn RoleRepo>,permission_repo: Arc<dyn PermissionRepo>)-> Self{
        Self { user_repo, role_repo , permission_repo }
    }
}
  
#[async_trait]
impl UserService for UserServiceImpl {
    
}
