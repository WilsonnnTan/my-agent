pub mod read_file;
pub mod write_file;
pub mod bash;
pub mod spec;

use read_file::ReadFile;
use write_file::WriteFile;
use bash::Bash;
use anyhow::{Result, bail};
use serde::Deserialize;

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