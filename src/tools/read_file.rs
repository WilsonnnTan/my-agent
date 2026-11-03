use std::{
  fs::OpenOptions,
  io::Read,
  borrow::Cow,
};
use serde::Deserialize;
use serde_json::{json, Value};
use anyhow::Result;
use schemars::{schema_for, JsonSchema};
use super::Tool;
use super::spec::Spec;

#[derive(Deserialize, JsonSchema)]
pub struct ReadFile<'a> {
  /// The path to the file to write to
  #[serde(borrow)]
  file_path: Cow<'a, str>,
}

impl<'a> Tool for ReadFile<'a> {
  fn run(&self) -> Result<String> {
    let mut file = OpenOptions::new()
                    .read(true)
                    .open(&*self.file_path)?;

    let mut data: String = String::new();
    let _ = file.read_to_string(&mut data);

    return Ok(data);
  }
}

impl<'a> Spec for ReadFile<'a> {
  fn spec() -> Value {
    let mut schema = schema_for!(ReadFile);

    let schema_obj = schema.as_object_mut().unwrap();
    schema_obj.remove("$schema");
    schema_obj.remove("title");

    let mut tool_spec = json!(
      {
        "type": "function",
        "function": {
          "name": "ReadFile",
          "description": "Read and return the contents of a file",
        }
      }
    );
    tool_spec["function"]["parameters"] = schema.to_value();
    tool_spec
  }
}