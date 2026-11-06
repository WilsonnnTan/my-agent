use super::prelude::*;
use std::{
  str,
  process::Command,
};
use anyhow::Context;


// use Cow to avoid unnecessary memory allocation
// Cow also allow mem allocation when the value of the str contains escaped character
#[derive(Deserialize, JsonSchema)]
pub(crate) struct Bash<'a> {
  /// The command to execute
  #[serde(borrow)]
  command: Cow<'a, str>,
}

impl<'a> Tool for Bash<'a> {
  fn run(&self) -> Result<String> {
    #[cfg(windows)]
    let (shell, flag) = ("cmd", "/C");

    #[cfg(unix)]
    let (shell, flag) = ("sh", "-c");

    let output = Command::new(shell)
      .arg(flag)
      .arg(&*self.command)
      .output()
      .context("Bash: failed to run command")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut concated_output = String::new();
    concated_output.push_str(&*stdout);
    concated_output.push_str(&*stderr);

    Ok(concated_output)
  }
}

impl<'a> Spec for Bash<'a> {
  fn spec() -> Value {
    let mut schema = schema_for!(Bash);

    let schema_obj = schema.as_object_mut().unwrap();
    schema_obj.remove("$schema");
    schema_obj.remove("title");

    let mut tool_spec = json!(
      {
        "type": "function",
        "function": {
          "name": "Bash",
          "description": "Execute a shell command",
        }
      }
    );
    tool_spec["function"]["parameters"] = schema.to_value();
    tool_spec
  }
}