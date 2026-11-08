use super::prelude::*;
use crate::{agent::Agent, skill::skill_loader::SkillLoader};

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
  async fn run(&self) -> Result<String> {
    let skills = SkillLoader::init()?;

    let args: Vec<&str> = self.args
                            .as_deref()
                            .map(|s| s.split_whitespace().collect())
                            .unwrap_or_default();

    let Some(skill) = skills.get(&*self.name) else {
      return Ok(String::from("Skill not found!"));
    };

    let skill_content = skill.get_skill_content(&args);

    // spawn sub-agent
    if skill.context.as_deref() == Some("fork") {
      // build message
      let mut messages: Vec<Value> = vec![
        json!(
          {
            "role": "user",
            "content": skill_content,
          }
        )
      ];
      // Run agent loop
      return Box::pin(Agent::run_loop(&mut messages)).await;
    }

    Ok(skill_content)
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