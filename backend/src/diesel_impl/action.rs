use diesel::prelude::*;
use crate::domain::permission::repo::{Action, Type};
use super::schema::{actions};
use std::str::FromStr;

#[derive(Debug,Queryable,Selectable)]
#[diesel(table_name=actions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ActionDiesel{
    pub id: i32,
    pub key: String,//CREATE,UPDATE,DELETE,READ
    pub description: Option<String>
}

impl From<ActionDiesel> for Action{
    fn from(value: ActionDiesel) -> Self {
        Action { 
            id: value.id, 
            key: Type::from_str(&value.key).unwrap_or(Type::READ),
            description: value.description.unwrap_or("".to_owned())
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
