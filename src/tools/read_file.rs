use super::prelude::*;
use std::{
  fs::OpenOptions,
  io::Read,
};


// use Cow to avoid unnecessary memory allocation
// Cow also allow mem allocation when the value of the str contains escaped character
#[derive(Deserialize, JsonSchema)]
pub(crate) struct ReadFile<'a> {
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
    file.read_to_string(&mut data)?;

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