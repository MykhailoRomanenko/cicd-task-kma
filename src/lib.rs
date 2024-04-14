mod config;
mod routes;
mod services;

use std::{net::Ipv4Addr, sync::Arc};

use axum::Router;
pub use config::AppConfig;
use services::hello::HelloService;

pub async fn run(config: AppConfig) {
    let app = bootstrap();
    let port = config.server_port;

    let addr = (Ipv4Addr::new(0, 0, 0, 0), port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn bootstrap() -> Router {
    let hello_service: Arc<_> = HelloService.into();

    routes::router(hello_service)
}
