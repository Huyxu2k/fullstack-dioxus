pub(crate) mod layer;

use crate::app_axum::state::AppState;
use async_trait::async_trait;
use axum::{extract::{Request, State}, response::Response};
use futures_util::future::BoxFuture;
use std::{
    sync::Arc, task::{Context, Poll}
};
use tower::{Layer, Service};

pub const AUTHORIZATION_HEADER: &str = "Authorization";
pub const BEARER: &str = "Bearer ";

#[async_trait]
pub trait THandler: Send + Sync {
    async fn handle_request<B>( req: Request<B>,state: State<Arc<AppState>>) -> Result<Request<B>, Response>
    where 
        B: Send;

}

#[derive(Clone)]
pub struct TLayer<T> {
    state: State<Arc<AppState>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> TLayer<T> {
    pub fn new(state: State<Arc<AppState>>) -> Self {
        Self {
            state: state.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S, T> Layer<S> for TLayer<T> {
    type Service = TMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        TMiddleware {
            inner,
            state: self.state.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct TMiddleware<S, T> {
    inner: S,
    state: State<Arc<AppState>>,
    _marker: std::marker::PhantomData<T>,
}

impl<S, T> Service<Request> for TMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Send + Clone + 'static,
    S::Future: Send +'static,
    T: THandler + Send + Sync,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response,Self::Error>>;


    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let state = self.state.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let result = T::handle_request(req,state).await;
            match result {
                Ok(modified_req) => {
                    let response = inner.call(modified_req).await?;
                    Ok(response)
                }
                Err(response) => Ok(response),
            }
        })
    }
}
