
use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{app_axum::{error::ApiError, state::AppState}, domain::user::repo::User};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email_or_username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

pub struct AuthHandler;

impl AuthHandler {
    pub async fn login(
        state: State<Arc<AppState>>,
        Json(data): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, ApiError> {
        let auth_service= state.auth_service.clone();
        let (user, token) = auth_service
            .login(&data.email_or_username, &data.password)
            .await?;

        let rep = LoginResponse { user, token };

        Ok(Json(rep))
    }

    pub async fn register( state: State<Arc<AppState>>,
        Json(data): Json<CreateUserRequest>,)-> Result<Json<LoginResponse>, ApiError> {
            let auth_service= state.auth_service.clone();
            let (user, token) = auth_service
                .register(data.username, data.email,data.password)
                .await?;
    
            let rep = LoginResponse { user, token };
    
            Ok(Json(rep))
    }
}
