use std::collections::HashMap;
use super::skill::Skill;


/// Parse prompt:
/// - Skill invocation
/// - Pass arguments to a skill
pub fn parse_prompt(prompt: String, skills: &HashMap<String, Skill>) -> String {
  let mut parsed_prompt: String = String::new();

  let mut args = prompt.split_whitespace().peekable();
  let mut invoked_skills: Vec<&Skill> = Vec::new(); // store list of skills that got invoked

  // Skill invocation
  while let Some(arg) = args.peek() {
    let invoked: Option<&Skill> = arg
                  .strip_prefix("/")
                  .and_then(|name| skills.get(name));
    
    match invoked {
      Some(s) => {
        invoked_skills.push(s);
        args.next();
      },
      None => { break },
    }
  }

  match invoked_skills.is_empty() {
    true => parsed_prompt = prompt,
    false => {
      let args: Vec<&str> = args.collect(); // scrap the rest of the args
      
      // Push skill's content that is formatted and replaced with args
      for skill in invoked_skills {
        parsed_prompt.push_str(&skill.get_skill_content(&args));
        parsed_prompt.push('\n');
      }
    },

  }

  parsed_prompt
}
