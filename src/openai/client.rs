use async_openai::{Client, config::OpenAIConfig};
use serde_json::{Value, json};
use super::super::config::Config;
use anyhow::Result;

pub struct LlmClient<'a> {
  /// openai client
  client: Client<OpenAIConfig>,
  /// Model name
  model: &'a str,
}

impl<'a> LlmClient<'a> {
  pub fn new(config: &'a Config) -> Self {
    let open_ai_config = OpenAIConfig::new()
                            .with_api_base(&config.base_url)
                            .with_api_key(&config.api_key);

    Self { 
      client: Client::with_config(open_ai_config),
      model: &config.model,
    }
  }

  pub async fn chat(&self, messages: &Vec<Value>) -> Result<Value>{
    let response: Value = self.client
      .chat()
      .create_byot(json!({
        "messages": messages,
        "model": self.model,
        "tools": [
          {
            "type": "function",
            "function": {
              "name": "Read",
              "description": "Read and return the contents of a file",
              "parameters": {
                "type": "object",
                "properties": {
                  "file_path": {
                    "type": "string",
                    "description": "The path to the file to read"
                  }
                },
                "required": ["file_path"]
              }
            }
          }
        ],
      })).await?;
    Ok(response)
  }
}
