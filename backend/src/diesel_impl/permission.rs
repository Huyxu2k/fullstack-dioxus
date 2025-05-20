use diesel::prelude::*;
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

// fn to_ids(data: String)->Vec<i32>{
//     data.split(',')
//         .map(|s| s.trim().parse::<i32>().unwrap_or(0) )
//         .collect::<Vec<_>>()
// }
// fn get_actions(ids: Vec<i32>)->Vec<Action>{
    
// }


impl Into<Permission> for PermissionDiesel{
    fn into(self) -> Permission {
        Permission {
            id: self.id,
            resource: self.resource ,
            action: Vec::new(),
            description: self.description.unwrap_or("".to_owned()),
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
}

#[async_trait::async_trait]
impl PermissionRepo for PermissionDieselImpl {
    async fn get(&self) -> Result<Vec<Permission>, RepoError>{
        todo!()
    }
    async fn get_permissions_by_role_id(&self, role_id: i32)->Result<Vec<Permission>, RepoError>{
        todo!()
    }
    async fn get_actions_by_ids(&self, ids: Vec<i32>)-> Result<Vec<Action>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = actions::table
                .filter(actions::id.eq_any(ids))
                .load::<ActionDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e));

            result.map(|actions| actions.into_iter().map(|v| v.into()).collect())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
}