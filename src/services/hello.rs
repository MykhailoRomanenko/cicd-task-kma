pub struct HelloService;

impl HelloService {
    pub async fn say_hello(&self) -> &'static str {
        "Hello, World!"
    }
}

#[cfg(test)]
mod tests {
    use super::HelloService;

    // Example unit test
    #[tokio::test]
    async fn should_say_hello() {
        let service = HelloService;

        let hello = service.say_hello().await;

        assert_eq!(hello, "Hello, World!");
    }
}
