use async_openai::{Client, config::OpenAIConfig};
use serde::Serialize;
use serde_json::Value;
use super::super::config::Config;
use anyhow::Result;
use super::super::tools::spec::TOOLS_SPEC;

pub struct LlmClient<'a> {
  /// openai client
  client: Client<OpenAIConfig>,
  /// Model name
  model: &'a str,
}

#[derive(Serialize)]
pub struct ChatRequest<'a> {
  /// List of messages including history
  messages: &'a Vec<Value>,
  /// Model name
  model: &'a str,
  /// List of tool's spec
  tools: &'a [Value],
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
      .create_byot(
        ChatRequest {
          messages: messages,
          model: self.model,
          tools: &*TOOLS_SPEC,
        }
      ).await?;
    Ok(response)
  }
}
