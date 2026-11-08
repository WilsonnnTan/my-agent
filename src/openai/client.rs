use std::sync::OnceLock;
use async_openai::{
  Client,
  config::OpenAIConfig
};
use serde::Serialize;
use serde_json::Value;
use anyhow::Result;
use crate::{
  config::Config,
  tools::TOOLS_SPEC,
};


static LLMCLIENT: OnceLock<LlmClient> = OnceLock::new();

pub struct LlmClient {
  /// openai client
  client: Client<OpenAIConfig>,
  /// Model name
  model: &'static str,
}

#[derive(Serialize, Debug)]
struct ChatRequest<'a> {
  /// List of messages including history
  messages: &'a Vec<Value>,
  /// Model name
  model: &'a str,
  /// List of tool's spec
  tools: &'a [Value],
}

impl LlmClient {
  pub fn init() -> &'static LlmClient {
    let config = Config::init();

    LLMCLIENT.get_or_init(|| {
      let open_ai_config = OpenAIConfig::new()
        .with_api_base(&config.base_url)
        .with_api_key(&config.api_key);

      LlmClient { 
        client: Client::with_config(open_ai_config),
        model: &config.model,
      }
    })
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
