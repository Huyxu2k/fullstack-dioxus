use common_model::user::{CreateUserRequest, FilterUserRequest};
use diesel::prelude::*;
use crate::domain::error::RepoError;
use crate::domain::user::repo::{User, UserRepo};

use super::schema::users;
use super::pool::{self, DbConn};
use std::sync::Arc;
use chrono::NaiveDateTime;

#[derive(Debug,Queryable,Selectable)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserDiesel{
    pub id: i32,
    pub employee_id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>
}

impl Into<User> for UserDiesel {
    fn into(self) -> User {
        User { 
            id: self.id, 
            employee_id: self.employee_id.unwrap_or(0),
            username: self.username, 
            password_hash: self.password_hash, 
            email: self.email, 
            is_active: self.is_active.unwrap_or(false), 
            created_at: self.created_at.unwrap_or(NaiveDateTime::default()) 
        }
    }
}

impl From<User> for UserDiesel {
    fn from(value: User) -> Self {
        UserDiesel{
            id: value.id,
            employee_id: Some(value.employee_id),
            username: value.username,
            password_hash: value.password_hash,
            email: value.email,
            is_active: Some(value.is_active),
            created_at: Some(value.created_at),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub employee_id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}


// impl repo

pub struct UserDieselImpl {
    pool: Arc<DbConn>,
}

impl UserDieselImpl {
    pub fn new(pool: Arc<DbConn>)-> Self{
        UserDieselImpl { pool }
    }
}

#[async_trait::async_trait]
impl UserRepo for UserDieselImpl {
    async fn get(&self, filter: FilterUserRequest) -> Result<Vec<User>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = users::table
                //.limit(filter.pagination.limit as i64)
                //.offset(filter.pagination.offset as i64)
                .load::<UserDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e));

            result.map(|users| users.into_iter().map(|v| v.into()).collect())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn get_by_id(&self, id: i32) -> Result<User, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = users::table
                .find(id)
                .first::<UserDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e));

            result.map(|user| user.into())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = users::table
                .filter(users::username.eq(email_or_username.clone()))
                .or_filter(users::email.eq(email_or_username))
                .first::<UserDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e));

            result.map(|user| user.into())
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
    async fn create(&self, username: String, email: String, password_hash: String) -> Result<User, RepoError>{
        let pool = self.pool.clone();
        let inserted_id = pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let new_user = NewUser { 
                employee_id: None, 
                username: username, 
                password_hash: password_hash, 
                email: email, 
                is_active: Some(true), 
                created_at: Some(chrono::Utc::now().naive_utc()) 
            };

            let result = diesel::insert_into(users::table)
                .values(&new_user)
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
    async fn update(&self, id: i32, user: User) -> Result<User, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            let result = diesel::update(users::table.find(id))
                .set(users::username.eq(user.username))
                .execute(&mut conn)
                .map_err(|e| RepoError::from(e))?;

            if result == 0 {
                return Err(RepoError{message:"Can't updated".to_string()});
            }
            let user_update = users::table
                .find(id)
                .first::<UserDiesel>(&mut conn)
                .map_err(|e| RepoError::from(e))?;

            Ok(user_update.into())
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

            let result = diesel::update(users::table.find(id.clone()))
                .set(users::is_active.eq(false))
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
    async fn delete_list_ids(&self, id: Vec<i32>) -> Result<Vec<i32>, RepoError>{
        let pool = self.pool.clone();
        pool::run(move || {
            let mut conn = pool
                .get()
                .map_err(|e| RepoError::from(e))?;

            for id in id.clone(){
                 diesel::update(users::table.find(id))
                .set(users::is_active.eq(false))
                .execute(&mut conn)
                .map_err(|e| RepoError::from(e))?;
            }

            Ok(id)
        })
        .await
        .map_err(|e| RepoError::from(e))?
    }
}