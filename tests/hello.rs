use cicd_task_kma::AppConfig;

// Example integration test
#[tokio::test]
async fn should_return_hello() {
    let port = 3000;
    let config = AppConfig { server_port: port };
    let _ = tokio::spawn(async move { cicd_task_kma::run(config).await });

    let response = reqwest::get(format!("http://localhost:{port}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(response, "Hello, World!");
}
