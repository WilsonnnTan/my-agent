use std::collections::HashMap;
use super::skill::Skill;


/// Parse prompt:
/// - Skill invocation
/// - Pass arguments to a skill
pub fn parse_prompt<'a>(prompt: &'a str, skills: &'a HashMap<String, Skill>) -> &'a str {
  // Skill invocation
  let invoked_skill: Option<&str> = prompt
                .strip_prefix("/")
                .and_then(|name| skills.get(name))
                .and_then(|skill| skill.content.as_deref());


  match invoked_skill {
    Some(s) => {
      let args: Vec<&str> = prompt.split_whitespace().collect();
      replace_args(s, args)
    },
    None => {
      // fallback to original prompt
      prompt
    },
  }
}


/// Implement args replacement on invoked skill
/// source: https://code.claude.com/docs/en/skills#available-string-substitutions
pub fn replace_args<'a>(invoked_skill: &'a str, args: Vec<&str>) -> &'a str {
  todo!()
}