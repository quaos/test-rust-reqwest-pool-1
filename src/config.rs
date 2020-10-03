use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
  pub host: String,
  pub port: i32,
  pub client_connect_timeout: Duration,

  pub google_search_api_engine_id: String,
  pub google_search_api_key: String,
}

impl AppConfig {
  pub fn from_env() -> Self {
    let client_connect_timeout_millis = std::env::var("APP_CLIENT_CONNECT_TIMEOUT")
      .map(|s| s.parse::<u64>().expect("failed to parse env as i32: APP_CLIENT_CONNECT_TIMEOUT"))
      .unwrap_or(30000u64);

    Self {
      host: std::env::var("APP_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string()),
      port: std::env::var("APP_PORT")
      .map(|s| s.parse::<i32>().expect("failed to parse env as i32: APP_PORT"))
        .unwrap_or(8080),
      client_connect_timeout: Duration::from_millis(client_connect_timeout_millis),
      google_search_api_engine_id: std::env::var("GOOGLE_SEARCH_API_ENGINE_ID")
        .unwrap_or_else(|_| "".to_string()),
      google_search_api_key: std::env::var("GOOGLE_SEARCH_API_KEY")
        .unwrap_or_else(|_| "".to_string()),
    }
  }
}
