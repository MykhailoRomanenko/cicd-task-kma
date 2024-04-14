use crate::services::hello::HelloService;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

#[derive(Clone)]
struct HelloRouterState {
    hello_service: Arc<HelloService>,
}

async fn handle_hello(
    State(HelloRouterState { hello_service }): State<HelloRouterState>,
) -> impl IntoResponse {
    hello_service.say_hello().await
}

pub fn router(hello_service: Arc<HelloService>) -> Router {
    let state = HelloRouterState { hello_service };
    Router::new()
        .route("/", get(handle_hello))
        .with_state(state)
}
