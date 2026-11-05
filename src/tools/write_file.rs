use std::{
  fs::OpenOptions,
  io::Write,
  borrow::Cow,
  path::Path,
};
use serde::Deserialize;
use serde_json::{json, Value};
use anyhow::Result;
use schemars::{schema_for, JsonSchema};
use super::{
  Tool,
  Spec,
};

// use Cow to avoid unnecessary memory allocation
// Cow also allow mem allocation when the value of the str contains escaped character
#[derive(Deserialize, JsonSchema)]
pub struct WriteFile<'a> {
  /// The path to the file to write to
  #[serde(borrow)]
  file_path: Cow<'a, str>,
  /// The content to write to the file
  #[serde(borrow)]
  content: Cow<'a, str>,
}

impl<'a> Tool for WriteFile<'a> {
  fn run(&self) -> Result<String> {
    let is_exist = Path::new(&*self.file_path).exists();

    let mut file = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&*self.file_path)?;

    file.write_all(self.content.as_bytes())?;

    Ok(String::from(format!("{} {} {}", is_exist, &*self.file_path, self.content.len())))
  }
}

impl<'a> Spec for WriteFile<'a> {
  fn spec() -> Value {
    let mut schema = schema_for!(WriteFile);

    let schema_obj = schema.as_object_mut().unwrap();
    schema_obj.remove("$schema");
    schema_obj.remove("title");

    let mut tool_spec = json!(
      {
        "type": "function",
        "function": {
          "name": "WriteFile",
          "description": "Write content to a file",
        }
      }
    );
    tool_spec["function"]["parameters"] = schema.to_value();
    tool_spec
  }
}