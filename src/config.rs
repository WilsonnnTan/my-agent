use async_openai::{config::OpenAIConfig};
use dotenvy::dotenv;
use std::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {
  /// OpenAIConfig with base_url and api_key
  pub open_ai_config: OpenAIConfig,
  /// Model name
  pub model: String,
}

impl Config {
  /// Reads configuration from the .env file (not from process environment).
  ///
  /// `BASE_URL` and `API_KEY` are required;
  /// `MODEL` (default `anthropic/claude-haiku-4.5`)
  pub fn load() -> Self {
    // Loads from .env file
    let var = Self::load_from_env().unwrap_or_else(|e| {
      eprintln!("Failed to load environment variables: {}", e);
      std::process::exit(1);
    });

    // Construct OpenAIConfig
    let open_ai_config = OpenAIConfig::new()
                          .with_api_base(var.0)
                          .with_api_key(var.1);
    
    Self {
      open_ai_config: open_ai_config,
      model: var.2
    }
  }

  /// Loads configuration from the .env files (not from process environment).
  pub fn load_from_env() -> Result<(String, String, String)> {
    dotenv()?;

    let base_url = env::var("BASE_URL")
                    .unwrap_or_else(|_| {
                      eprintln!("BASE_URL is not set");
                      std::process::exit(1);
                    });

    let api_key = env::var("API_KEY")
                    .unwrap_or_else(|_| {
                      eprintln!("API_KEY is not set");
                      std::process::exit(1);
                    });

    let model = env::var("MODEL")
                    .unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string());
    
    Ok((base_url, api_key, model))
  }
}