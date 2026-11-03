pub mod read_file;
pub mod spec;

use read_file::ReadFile;
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
///    "type": "function",
///     "function": {
///       "name": "Read",
///       "arguments": "{\"file_path\": \"/path/to/file.txt\"}"
///     }
///   }
/// ]
/// ```
// PERF: can be improved by using borrow
#[derive(Deserialize)]
pub struct ToolCall{
  pub id: String,
  r#type: String,
  function: Function,
}

#[derive(Deserialize)]
pub struct Function {
  name: String,
  arguments: String,
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
      ("function", "Read") => {
        let tool: ReadFile = serde_json::from_str(self.function.arguments.as_str())?;
        tool.run()
      },
      _ => { bail!(format!("{} tool doesn't exist in current program", self.function.name))},
    }
  }
}