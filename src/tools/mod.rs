pub mod read_file;

use read_file::ReadFile;
use anyhow::{Result, bail};

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
/// 
/// 'tool_calls' field need to be parsed first before passing to 'ToolCall' Struct
/// for 'function' field, it will be deserialize inside 'ToolCall::call()'
pub struct ToolCall<'a> {
  r#type: &'a str,
  name: &'a str,
  arguments: &'a str,
}

impl<'a> ToolCall<'a> {
  pub fn new(r#type: &'a str, name: &'a str, arguments: &'a str) -> Self {
    Self { r#type, name, arguments }
  }

  pub fn call(&self) -> Result<String>{
    match (self.r#type, self.name) {
      ("function", "Read") => {
        let tool: ReadFile = serde_json::from_str(self.arguments)?;
        tool.run()
      },
      _ => { bail!(format!("{} tool doesn't exist in current program", self.name))},
    }
  }
}