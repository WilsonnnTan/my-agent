use dotenvy::dotenv;
use std::{env, sync::OnceLock};


static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Config {
  /// OpenAI Compatible Provider `BASE_URL`
  pub base_url: String,
  /// OpenAI Compatible Provider `API_KEY`
  pub api_key: String,
  /// Model name
  pub model: String,
}

impl Config {
  /// Reads configuration from the .env file (not from process environment)
  /// and load it globally.
  /// 
  /// `BASE_URL` and `API_KEY` are required;
  /// `MODEL` (default `anthropic/claude-haiku-4.5`)
  pub fn init() -> &'static Config {
    CONFIG.get_or_init(Self::load_from_env)
  }

  /// Loads configuration from the .env files (not from process environment).
  fn load_from_env() -> Self {
    if let Err(_) = dotenv() {} // ignore if there is no .env file

    let base_url = env::var("OPENROUTER_BASE_URL")
                    .unwrap_or_else(|_| {
                      eprintln!("OPENROUTER_BASE_URL is not set");
                      std::process::exit(1);
                    });

    let api_key = env::var("OPENROUTER_API_KEY")
                    .unwrap_or_else(|_| {
                      eprintln!("OPENROUTER_API_KEY is not set");
                      std::process::exit(1);
                    });

    let model = env::var("MODEL")
                    .unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string());
    
    Self {
      base_url,
      api_key,
      model
    }
  }
}