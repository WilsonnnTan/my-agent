pub mod read_file;
pub mod write_file;
pub mod bash;

use read_file::ReadFile;
use write_file::WriteFile;
use bash::Bash;
use anyhow::{Result, bail};
use serde::Deserialize;
use serde_json::Value;
use std::sync::LazyLock;


/// Parse tool_calls field on AI response
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

trait Tool {
  fn run(&self) -> Result<String>;
}

/// example tool call json structure:
/// 
/// ```json
/// "tool_calls": [
///   {
///     "id": "call_abc123",
///     "type": "function",
///     "function": {
///       "name": "Read",
///       "arguments": "{\"file_path\": \"/path/to/file.txt\"}"
///     }
///   }
/// ]
/// ```
///
// Use Cow to decode the escaped character on `function.arguments`
// Why? because `function.arguments` is a JSON string
// which is the reason why it will have escaped character
// PERF: can be improved by using borrow
#[derive(Deserialize)]
pub struct ToolCall{
  pub id: String,
  pub r#type: String,
  pub function: Function,
}

#[derive(Deserialize)]
pub struct Function {
  pub name: String,
  pub arguments: String,
}

impl ToolCall {
  pub fn new(id:String, r#type: String, name: String, arguments: String) -> Self {
    Self { 
      id: id,
      r#type: r#type,
      function: Function {
        name: name,
        arguments: arguments
      }
    }
  }

  pub fn call(&self) -> Result<String>{
    match (self.r#type.as_str(), self.function.name.as_str()) {
      ("function", "ReadFile") => {
        let tool: ReadFile = serde_json::from_str(self.function.arguments.as_str())?;
        tool.run()
      },
      ("function", "WriteFile") => {
        let tool: WriteFile = serde_json::from_str(self.function.arguments.as_str())?;
        tool.run()
      },
      ("function", "Bash") => {
        let tool: Bash = serde_json::from_str(self.function.arguments.as_str())?;
        tool.run()
      },
      _ => { bail!(format!("{} tool doesn't exist in current agent", self.function.name))},
    }
  }
}


/// Make sure spec() implementation uses unwrap
/// so the program doesn't run if there is error on tool's spec
pub trait Spec {
  fn spec() -> Value;
}

/// `TOOLS` contains the list of tools spec exist in this project
pub static TOOLS_SPEC: LazyLock<[Value; 3]> = LazyLock::new(|| {
  [
    ReadFile::spec(),
    WriteFile::spec(),
    Bash::spec(),
  ]
});