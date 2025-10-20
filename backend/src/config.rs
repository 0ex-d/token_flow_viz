#[derive(Clone)]
pub struct AppConfig {
    pub hypersync_api_key: Option<String>,
    pub server_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let hypersync_api_key = std::env::var("HYPERSYNC_API_KEY").ok();
        let server_url = std::env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8789".to_string());
        Ok(Self { hypersync_api_key, server_url })
    }
}
