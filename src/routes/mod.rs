use crate::services::hello::HelloService;
use axum::Router;
use std::sync::Arc;

mod hello;

pub fn router(hello_service: Arc<HelloService>) -> Router {
    Router::new().merge(hello::router(hello_service))
}
