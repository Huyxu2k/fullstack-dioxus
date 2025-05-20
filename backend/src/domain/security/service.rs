use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{user::repo::{User,UserRepo}};
use crate::domain::error::CommonError;

use super::repo::{SecurityService, Token};




#[async_trait]
pub trait AuthService:Sync + Send {
    async fn login(&self, email_or_username: &str, password: &str) -> Result<(User, Token), CommonError>;
    async fn logout(&self,user_id:i32);
    //async fn refresh_token(&self, user_id: i32, refresh_token: &str)->Result<(User,String),CommonError>;
}


pub struct AuthServiceImpl{
  pub user_repo: Arc<dyn UserRepo>,
  pub security_service: Arc<dyn SecurityService>
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email_or_username: &str, password: &str) -> Result<(User, Token), CommonError>{
        let user= self.user_repo.get_by_email_or_username(email_or_username.to_string()).await.map_err(|e|e.into())?;

        if !(self.security_service.hash(password).await?==user.password_hash){
            return Err(CommonError { message:"Password or username is incorrected".to_string() , code: 1 });
        }
        let token=self.security_service.create_jwt(&user).await.map_err(|e|e.into())?;
        //save token

        Ok((user,token))
    }
    async fn logout(&self,user_id:i32){

        //revoked token
        todo!()
    }
}