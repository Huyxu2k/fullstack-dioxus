use diesel::prelude::*;
use futures_util::future::join_all;
use crate::domain::error::RepoError;
use crate::domain::permission::repo::{Permission,PermissionRepo,Action};
use super::action::ActionDiesel;
use super::schema::{permissions, role_permissions, actions};
use super::pool::{self, DbConn};
use std::sync::Arc;

#[derive(Debug,Queryable,Selectable)]
#[diesel(table_name=permissions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PermissionDiesel{
    pub id: i32,
    pub resource: String,
    pub action: String,//1,2,3,4
    pub description: Option<String>
}

impl PermissionDiesel {
    fn to_action_ids(&self) -> Vec<i32> {
        self.action
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok()) 
            .collect()
    }
}


impl From<PermissionDiesel> for Permission{
    fn from(value: PermissionDiesel) -> Self {
        Permission {
            id: value.id,
            resource: value.resource,
            action: Vec::new(),
            description: value.description.unwrap_or("".to_owned()),
        }
    }
}

// impl repo

pub struct PermissionDieselImpl{
    pool: Arc<DbConn>,
}

impl PermissionDieselImpl {
    pub fn new(pool: Arc<DbConn>)->Self{
        PermissionDieselImpl {pool}
    }
    async fn get_actions_by_ids(&self, ids: Vec<i32>) -> Result<Vec<Action>, RepoError> {
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool.get()?;

            actions::table
                .filter(actions::id.eq_any(ids))
                .load::<ActionDiesel>(&mut conn)
                .map(|actions| actions.into_iter().map(|v| Ok(v.into())).collect())?
        })
        .await?
    }
    async fn map_to_permission(
        &self,
        permission_diesel: PermissionDiesel,
    ) -> Result<Permission, RepoError> {
        let action_ids = permission_diesel.to_action_ids();
        let actions = self.get_actions_by_ids(action_ids).await?;
        Ok(Permission {
            id: permission_diesel.id,
            resource: permission_diesel.resource,
            action: actions,
            description: permission_diesel.description.unwrap_or_default(),
        })
    }
}

#[async_trait::async_trait]
impl PermissionRepo for PermissionDieselImpl {
    async fn get(&self) -> Result<Vec<Permission>, RepoError> {
        let pool = self.pool.clone();

        let results = pool::run(move || {
            let mut conn = pool.get()?;
            permissions::table.load::<PermissionDiesel>(&mut conn).map_err(RepoError::from)
        })
        .await??;
    
        let tasks = results.into_iter().map(|permissions_diesel| {
            let self_clone = self;
            async move {
                self_clone.map_to_permission(permissions_diesel).await
            }
        });
    
        let permissions = join_all(tasks).await;
        permissions.into_iter().collect()
    }
    async fn get_permissions_by_role_id(&self, role_id: i32)->Result<Vec<Permission>, RepoError>{
        let pool = self.pool.clone();
        let results=pool::run(move || {
            let mut conn = pool.get()?;

            role_permissions::table
                .inner_join(permissions::table.on(role_permissions::permission_id.eq(permissions::id)))
                .filter(role_permissions::role_id.eq(role_id))
                .select(permissions::all_columns)
                .load::<PermissionDiesel>(&mut conn).map_err(RepoError::from)
        })
        .await??;

        let tasks = results.into_iter().map(|permissions_diesel| {
            let self_clone = self;
            async move {
                self_clone.map_to_permission(permissions_diesel).await
            }
        });
    
        let permissions = join_all(tasks).await;
        permissions.into_iter().collect()
    }
    async fn get_actions_by_ids(&self, ids: Vec<i32>)-> Result<Vec<Action>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool.get()?;

            let result = actions::table
                .filter(actions::id.eq_any(ids))
                .load::<ActionDiesel>(&mut conn)?;

            result.into_iter().map(|action| Ok(action.into())).collect()
        })
        .await?
    }
}