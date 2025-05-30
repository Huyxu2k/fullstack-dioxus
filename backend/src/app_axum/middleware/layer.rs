use async_trait::async_trait;
use std::sync::Arc;
use axum::{body::Body, extract::{Request, State}, http::StatusCode, response::Response};

use super::{THandler, AUTHORIZATION_HEADER, BEARER};
use crate::{app_axum::{error::ApiError, state::AppState}, domain::user::repo::UserIdentity};

// layer check token
#[derive(Debug, Clone)]
pub struct TokenLayer;

#[async_trait]
impl THandler for TokenLayer {
    async fn handle_request<B>(mut req: Request<B>, state: State<Arc<AppState>>) -> Result<Request<B>, Response>
    where 
        B:Send
    {
        let security_service = state.security_service.clone();
        let token = req
        .headers()
        .get(AUTHORIZATION_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix(BEARER))
        .ok_or_else(|| Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap())?;

        let result: Result<Request<B>, Response> = match security_service.verify_jwt(token.to_string()).await.map_err(|e| ApiError::from(e)){
            Ok(value) => {
               if value{
                let claim= security_service.decode(token).await.unwrap();
                let identity = UserIdentity {
                    email: claim.claims.email.to_string(),
                    user_id: claim.claims.sub as i32,
                };
                req.extensions_mut().insert(identity); 
                
                Ok(req)
               } else{
                Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Token verification failed"))
                .unwrap())
               }
            },
            Err(e) => Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Token verification failed"))
                .unwrap()),
        }; 
        result
    }
}


// layer check role
#[derive(Debug, Clone)]
pub struct AuthorizationLayer;

#[async_trait]
impl THandler for AuthorizationLayer {
    async fn handle_request<B>(req: Request<B>, State(state): State<Arc<AppState>>) -> Result<Request<B>, Response> 
    where 
        B:Send
    {
        let role_header = req.headers().get("Role");

        let response = Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Forbidden: You do not have the required permissions"))
            .unwrap();

        if let Some(role) = role_header {
            if let Ok(role_str) = role.to_str() {
                let role_owned = role_str.to_owned(); 
                if role_owned == "admin" {
                    return Ok(req);
                }
            }
        }
        Err(response)
    }
}