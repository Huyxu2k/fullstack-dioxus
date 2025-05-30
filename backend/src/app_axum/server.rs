use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, routing::{get, post}, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use super::{handler::{auth::AuthHandler, health::health_check}, middleware::{layer::{AuthorizationLayer, TokenLayer}, TLayer}, state::AppState};

type _TokenLayer = TLayer<TokenLayer>;
type _AuthorizationLayer = TLayer<AuthorizationLayer>;
fn app_routes(state: State<Arc<AppState>>)->Router{

    // init layer
    let token_layer= _TokenLayer::new(state.clone());
    let authorization_layer= _AuthorizationLayer::new(state.clone());
    let cors_layer= CorsLayer::new()
                                        .allow_methods([axum::http::Method::GET,axum::http::Method::POST])
                                        .allow_headers([
                                            axum::http::header::CONTENT_TYPE,
                                            axum::http::header::AUTHORIZATION,
                                        ])
                                        .allow_credentials(true);

    // verifi token
    let level_token=ServiceBuilder::new()
                        .layer(token_layer);

    // các route của các module
    // let module_routes = Router::new()
    //                     .nest("/users", user_router(state.clone()));            


    Router::new()
    .route("/health_check",get(health_check) )
    .route("/api/v1/login",post(AuthHandler::login))
    .route("/api/v1/register",post(AuthHandler::register))
    .with_state(state.0)
    
    //.nest("/api/v1/", module_routes)
    //.layer(level_2)
}


pub async fn start(){
    let state=Arc::new(AppState::new());
    let app= app_routes(State(state));

    let listener= tokio::net::TcpListener::bind("127.0.0.1:8086")
                .await
                .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()   
    )
    .await
    .unwrap();
}