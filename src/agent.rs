use serde_json::{Value, json};
use anyhow::Result;
use crate::{
  openai::client::LlmClient,
  tools::response_tool_calls_parser
};


pub struct Agent;

/// Implementation of loop agent
impl Agent {
  pub async fn run_loop(messages: &mut Vec<Value>) -> Result<String> {
    let llm_client = LlmClient::init();

    loop {
      let response: Value = llm_client.chat(&messages).await?;

      // push AI response to messages to keep track of conversation
      if let Some(ai_message) = response.pointer("/choices/0/message") {
        messages.push(ai_message.clone());
      }

      let tool_calls = response_tool_calls_parser(&response)?;

      // break agent loop if `tool_calls` is empty or `finish_reason` is "stop"
      if tool_calls.is_empty() || response["choices"][0]["finish_reason"].as_str() == Some("stop") {
        if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
          return Ok(content.to_string()); // return agent final output
        }
        break;
      }

      for tool_call in tool_calls {
        let mut tool_message: Value = json!(
          {
            "role": "tool",
          }
        );

        match tool_call.call().await {
          Ok(output) => {
            tool_message["content"] = output.into();
          },
          Err(e) => {
            let error_message = format!("failed to call {} tool: {}", tool_call.function.name, e);
            // dbg!(error_message);
            tool_message["content"] = error_message.into();
          },
        }

        tool_message["tool_call_id"] = tool_call.id.into();
        messages.push(tool_message);
      }
    }

    Ok(String::new())
  }
}