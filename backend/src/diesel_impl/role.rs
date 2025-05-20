use diesel::prelude::*;
use crate::domain::error::RepoError;
use crate::domain::role::repo::{Role,RoleRepo};
use super::schema::{roles, user_roles};
use super::pool::{self, DbConn};
use std::sync::Arc;

#[derive(Debug,Queryable,Selectable)]
#[diesel(table_name=roles)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RoleDiesel{
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

impl Into<Role> for RoleDiesel{
    fn into(self) -> Role {
        Role { 
            id: self.id, 
            name: self.name, 
            description: self.description.unwrap_or("".to_owned())
        }
    }
}

impl From<Role> for RoleDiesel {
    fn from(value: Role) -> Self {
        RoleDiesel { 
            id: value.id, 
            name: value.name, 
            description: Some(value.description)
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>
}

// impl repo
pub struct RoleDieseImpl{
    pool: Arc<DbConn>,
}

impl RoleDieseImpl {
    pub fn new(pool: Arc<DbConn>)-> Self{
        RoleDieseImpl { pool }
    }
}

#[async_trait::async_trait]
impl RoleRepo for RoleDieseImpl {
    async fn get(&self) -> Result<Vec<Role>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = roles::table
                .load::<RoleDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e));

            result.map(|roles| roles.into_iter().map(|v| v.into()).collect())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn get_by_id(&self, id: i32) -> Result<Role, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = roles::table
                .find(id)
                .first::<RoleDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e));

            result.map(|role| role.into())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn create(&self, name: String, description: String) -> Result<Role, RepoError>{
        let pool = self.pool.clone();
        let inserted_id = pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let new_role = NewRole { name, description: Some(description) };

            let result = diesel::insert_into(roles::table)
                .values(&new_role)
                .execute(&mut conn)
                .map_err(|e| RepoError::from(e))?;
            if result == 0 {
                return Err(RepoError{message:"Can't inserted".to_string()});
            }
            let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result::<i32>(&mut conn)
            .map_err(|e| RepoError::from(e));
            id
        })
        .await
        .map_err(|e| RepoError::from(e))??;

        self.get_by_id(inserted_id).await.map_err(|e| RepoError::from(e))
    }
    async fn update(&self, id: i32, name: String, description: String) -> Result<Role, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = diesel::update(roles::table.find(id))
                .set((
                    roles::name.eq(name),
                    roles::description.eq(description),
                ))
                .execute(&mut conn)
                .map_err(|e| RepoError::from(e))?;

            if result == 0 {
                return Err(RepoError{message:"Can't updated".to_string()});
            }
            let role_update = roles::table
                .find(id)
                .first::<RoleDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e))?;

            Ok(role_update.into())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn delete_by_id(&self, id: i32) -> Result<i32, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = diesel::delete(roles::table.find(id.clone()))
                .execute(&mut conn)
                .map_err(|e| RepoError::from(e))?;

            if result==0{
                return Err(RepoError{message:"Can't Delete".to_string()});
            }
            Ok(id)
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn get_roles_by_user_id(&self, user_id: i32) -> Result<Vec<Role>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;
            let result= roles::table.inner_join(user_roles::table.on(user_roles::role_id.eq(roles::id)))
                            .filter(user_roles::user_id.eq(user_id))
                            .select((roles::id, roles::name, roles::description))
                            .load::<RoleDiesel>(&mut conn)
                            .map_err(|e| RepoError::from(e));
                        result.map(|roles| roles.into_iter().map(|v| v.into()).collect())
        })
        .await
        .map_err(|e| RepoError::from(e))?

    }
}