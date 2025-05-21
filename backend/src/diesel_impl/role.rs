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

impl From<RoleDiesel> for Role {
    fn from(value: RoleDiesel) -> Self {
        Role {
            id: value.id,
            name: value.name,
            description: value.description.unwrap_or_else(|| "".to_string()),
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
            let mut conn = pool.get()?;

            let result = roles::table.load::<RoleDiesel>(&mut conn)?;

            result.into_iter().map(|role| Ok(role.into())).collect()
        })
        .await?
    }
    async fn get_by_id(&self, id: i32) -> Result<Role, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool.get()?;
            let result = roles::table
                .find(id)
                .first::<RoleDiesel>(&mut conn)?;

            Ok(result.into())
        })
        .await?
    }
    async fn create(&self, name: String, description: String) -> Result<Role, RepoError>{
        let pool = self.pool.clone();
        let inserted_id = pool::run(move || {
            let mut conn = pool.get()?;

            let new_role = NewRole { name, description: Some(description) };

            let result = diesel::insert_into(roles::table)
                .values(&new_role)
                .execute(&mut conn)?;
            if result == 0 {
                return Err(RepoError{message:"Can't inserted".to_string()});
            }
            let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result::<i32>(&mut conn)?;
            
            Ok(id)
        })
        .await??;

        self.get_by_id(inserted_id).await
    }
    async fn update(&self, id: i32, name: String, description: String) -> Result<Role, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool.get()?;

            let result = diesel::update(roles::table.find(id))
                .set((
                    roles::name.eq(name),
                    roles::description.eq(description),
                ))
                .execute(&mut conn)?;

            if result == 0 {
                return Err(RepoError{message:"Can't updated".to_string()});
            }
            let role_update = roles::table
                .find(id)
                .first::<RoleDiesel>(&mut conn)?;

            Ok(role_update.into())
        })
        .await?
    }
    async fn delete_by_id(&self, id: i32) -> Result<i32, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool.get()?;

            let result = diesel::delete(roles::table.find(id.clone()))
                .execute(&mut conn)?;

            if result==0{
                return Err(RepoError{message:"Can't Delete".to_string()});
            }
            Ok(id)
        })
        .await?
    }
    async fn get_roles_by_user_id(&self, user_id: i32) -> Result<Vec<Role>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool.get()?;
            let result= roles::table.inner_join(user_roles::table.on(user_roles::role_id.eq(roles::id)))
                            .filter(user_roles::user_id.eq(user_id))
                            .select((roles::id, roles::name, roles::description))
                            .load::<RoleDiesel>(&mut conn)?;
            result.into_iter().map(|role| Ok(role.into())).collect()
        })
        .await?

    }
}