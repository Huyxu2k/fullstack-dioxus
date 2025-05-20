use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterUserRequest{

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest{
    pub user_name: String,
    pub email: String,
    pub password: String,
}