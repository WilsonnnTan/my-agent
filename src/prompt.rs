use std::collections::HashMap;
use super::skill::Skill;

pub fn parse_prompt<'a>(prompt: &'a str, skills: &'a HashMap<String, Skill>) -> &'a str {
  // parse prompt (check for skill invocation)
  let parsed_prompt: &str = prompt
                .strip_prefix("/")
                .and_then(|name| skills.get(name))
                .and_then(|skill| skill.content.as_deref())
                .unwrap_or(prompt);

  parsed_prompt
}