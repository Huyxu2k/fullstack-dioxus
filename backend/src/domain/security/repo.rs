use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::domain::error::{CommonError,RepoError};
use crate::domain::user::repo::User;

pub const EXPIRES: i64 = 24 * 60 * 60;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
    pub email: String,
    pub username: String
}

impl Claims {
    pub fn new(sub: i64, email: String, username: String) -> Self {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + EXPIRES;
        Self { sub, exp, iat ,email, username }
    }
}

#[derive(Clone,Debug, PartialEq)]
pub struct Token(String);


#[async_trait]
pub trait SecurityService: Send + Sync {
    async fn hash(&self, input: &str) -> Result<String, CommonError>;

    async fn verify_hash(&self, hashed: &str, raw: &str) -> Result<bool, CommonError>;

    async fn verify_jwt(&self, token: String)-> Result<bool, CommonError>;

    async fn create_jwt(&self, user: &User) -> Result<Token, CommonError>;

    async fn decode(&self, token: &str) -> Result<TokenData<Claims>, CommonError>;

    async fn encode(&self, claim: Claims) -> Result<String, CommonError>;
}
pub struct SecurityServiceImpl {
    pub key: String,
}

#[async_trait]
impl SecurityService for SecurityServiceImpl {
    async fn hash(&self, value: &str) -> Result<String, CommonError> {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    async fn verify_hash(&self, hashed: &str, pass: &str) -> Result<bool, CommonError> {
        let hashed_curr = self.hash(pass).await?;
        Ok(hashed_curr == hashed)
    }

    async fn verify_jwt(&self, token: String)-> Result<bool, CommonError>{
        let decode_claim = self.decode(&token).await.map_err(|e| e.into())?;
        if decode_claim.claims.exp >= chrono::Utc::now().timestamp() {
            Ok(true)
        } else {
            Err(RepoError{message: "Access token is expired!".to_string()}.into())
        }
    }

    async fn create_jwt(&self, user: &User) -> Result<Token, CommonError> {
        let time_origin = chrono::Utc::now();
        let now = time_origin.clone().timestamp();
        let exp = now + EXPIRES;
        let claim = Claims {
            sub: user.id.clone() as i64,
            exp,
            iat: now,
            email: user.email.clone(),
            username: user.username.clone()
        };
        let token = self
            .encode(claim.clone())
            .await
            .map_err(|e| e.into())?;

        Ok(Token(token))
    }

    async fn decode(&self, token: &str) -> Result<TokenData<Claims>, CommonError> {
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&self.key.as_ref()),
            &Validation::default()); //new(Algorithm::HS256)
        match result {
            Ok(decode) => Ok(decode),
            Err(e) =>Err(RepoError{message: format!("Error decode token: {}", e)}.into()),
        }
    }

    async fn encode(&self, claim: Claims) -> Result<String, CommonError> {
        let token=encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(&self.key.as_ref()),
        );
        match token {
            Ok(token) => Ok(token),
            Err(e) => Err(RepoError{message: format!("Error encode token: {}", e)}.into()),
        }
    }
}
