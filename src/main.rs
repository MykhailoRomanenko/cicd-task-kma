use cicd_task_kma::{run, AppConfig};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let config = AppConfig::load().expect("failed to load configuration");

    run(config).await
}
