use diesel::prelude::*;
use crate::domain::error::RepoError;
use crate::domain::permission::repo::{Action, Type};
use super::schema::{actions};
use super::pool::{self, DbConn};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug,Queryable,Selectable)]
#[diesel(table_name=actions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ActionDiesel{
    pub id: i32,
    pub key: String,//CREATE,UPDATE,DELETE,READ
    pub description: Option<String>
}

impl Into<Action> for ActionDiesel{
    fn into(self) -> Action {
        Action { 
            id: self.id, 
            key: Type::from_str(&self.key).unwrap(),
            description: self.description.unwrap_or("".to_owned())
        }
    }
}

impl From<Action> for ActionDiesel {
    fn from(value: Action) -> Self {
        ActionDiesel { 
            id: value.id, 
            key: value.key.to_string(), 
            description: Some(value.description)
        }
    }
}
