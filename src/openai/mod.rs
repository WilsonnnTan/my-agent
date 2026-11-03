pub mod client;

use serde_json::Value;
use anyhow::Result;
use crate::tools::ToolCall;


pub fn response_tool_calls_parser(response: &Value) -> Result<Vec<ToolCall>> {
  let mut parsed_tool_calls: Vec<ToolCall> = Vec::new();

  if let Some(tool_calls) = response
                            .pointer("/choices/0/message/tool_calls")
                            .and_then(|tool_calls| tool_calls.as_array())
                            .filter(|arr| !arr.is_empty())
  {
    for tool_call_field in tool_calls {
      let tool_call: ToolCall = serde_json::from_value(tool_call_field.clone())?;
      parsed_tool_calls.push(tool_call);
    }
  }
  Ok(parsed_tool_calls)
}