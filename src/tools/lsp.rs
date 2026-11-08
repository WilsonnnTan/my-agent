use super::prelude::*;


#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum LSPOperation {
  GoToDefinition
}

// use Cow to avoid unnecessary memory allocation
// Cow also allow mem allocation when the value of the str contains escaped character
#[derive(Deserialize, JsonSchema)]
pub(crate) struct LSP<'a> {
  operation: LSPOperation,
  /// Absolute path, or a path relative to the agent's current working directory
  #[serde(borrow)]
  file_path: Cow<'a, str>,
  /// 1-based line number of the symbol
  #[serde(borrow)]
  line: Cow<'a, str>,
  /// 1-based character offset in UTF-16 code units (same as character count for ASCII)
  #[serde(borrow)]
  character: Cow<'a, str>,
}

impl<'a> Tool for LSP<'a> {
  async fn run(&self) -> Result<String> {
    // TODO
    Ok("LSP is not implemented yet".to_string())
  }
}

impl<'a> Spec for LSP<'a> {
  fn spec() -> Value {
    let mut schema = schema_for!(LSP);

    let schema_obj = schema.as_object_mut().unwrap();
    schema_obj.remove("$schema");
    schema_obj.remove("title");

    let mut tool_spec = json!(
      {
        "type": "function",
        "function": {
          "name": "LSP",
          "description": "Returns the file, line, and character where a symbol is defined.",
        }
      }
    );
    tool_spec["function"]["parameters"] = schema.to_value();
    tool_spec
  }
}