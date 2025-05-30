use backend::app_axum::server::start;

#[tokio::main]
async fn main() {
    start().await;
}
