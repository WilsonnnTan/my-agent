use super::prelude::*;
use crate::skill::skill_loader::SkillLoader;

// use Cow to avoid unnecessary memory allocation
// Cow also allow mem allocation when the value of the str contains escaped character
#[derive(Deserialize, JsonSchema)]
pub(crate) struct InvokeSkill<'a> {
  /// The name of the skill to use
  #[serde(borrow)]
  name: Cow<'a, str>,
  /// Optional arguments for the skill
  #[serde(borrow)]
  args: Option<Cow<'a, str>>,
}

impl<'a> Tool for InvokeSkill<'a> {
  fn run(&self) -> Result<String> {
    let skills = SkillLoader::init()?;

    let args: Vec<&str> = self.args
                            .as_deref()
                            .map(|s| s.split_whitespace().collect())
                            .unwrap_or_default();

    match skills.get(&*self.name) {
      None => return Ok(String::from("Skill not found!")),
      Some(s) => return Ok(s.get_skill_content(&args)),
    }
  }
}

impl<'a> Spec for InvokeSkill<'a> {
  fn spec() -> Value {
    let mut schema = schema_for!(InvokeSkill);

    let schema_obj = schema.as_object_mut().unwrap();
    schema_obj.remove("$schema");
    schema_obj.remove("title");

    let mut tool_spec = json!(
      {
        "type": "function",
        "function": {
          "name": "InvokeSkill",
          "description": "Load a skill's instructions into the conversation",
        }
      }
    );
    tool_spec["function"]["parameters"] = schema.to_value();
    tool_spec
  }
}