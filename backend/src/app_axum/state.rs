use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime, TimeZone};

use crate::{application::{auth_service::{self, AuthService, AuthServiceImpl}, user_service::{UserService, UserServiceImpl}}, diesel_impl::pool::{db_pool, DbConn}, domain::{permission::repo::PermissionRepo, role::repo::RoleRepo, security::repo::SecurityService, user::repo::UserRepo}};


#[derive(Clone)]
pub struct AppState{
    pub auth_service: Arc<dyn AuthService>,
    pub security_service: Arc<dyn SecurityService>,
    pub user_service: Arc<dyn UserService>,
}

impl AppState {
    pub fn new()->AppState{
        let pool=Arc::new(db_pool());
        let user_repo: Arc<dyn UserRepo> = Arc::new(crate::diesel_impl::user::UserDieselImpl::new(pool.clone()));
        let security_service: Arc<dyn SecurityService> = Arc::new(crate::domain::security::repo::SecurityServiceImpl::new());
        let role_repo: Arc<dyn RoleRepo>=Arc::new(crate::diesel_impl::role::RoleDieselImpl::new(pool.clone()));
        let permission_repo: Arc<dyn PermissionRepo>=Arc::new(crate::diesel_impl::permission::PermissionDieselImpl::new(pool.clone()));

        let auth_service: Arc<dyn AuthService>=Arc::new(AuthServiceImpl::new(user_repo.clone(), security_service.clone()));
        let user_service: Arc<dyn UserService>=Arc::new(UserServiceImpl::new(user_repo.clone(), role_repo, permission_repo));

        AppState{
            auth_service,
            security_service,
            user_service,
        }
    }
}
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}