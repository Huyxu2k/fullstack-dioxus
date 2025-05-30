use axum::{body::Body, http::StatusCode, response::Response};



pub async fn health_check() -> Response {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Success"))
        .unwrap();

    response
}