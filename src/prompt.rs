use std::collections::HashMap;
use super::skill::Skill;


/// Parse prompt:
/// - Skill invocation
/// - Pass arguments to a skill
pub fn parse_prompt<'a>(prompt: String, skills: &'a HashMap<String, Skill>) -> String {
  let mut parsed_prompt: String = String::new();
  let mut args = prompt.split_whitespace().peekable();
  let mut have_invoked_skill: bool = false; // flag for replace_args trigger or same prompt

  // Skill invocation
  while let Some(arg) = args.peek() {
    let invoked_skill: Option<&str> = arg
                  .strip_prefix("/")
                  .and_then(|name| skills.get(name))
                  .and_then(|skill| skill.content.as_deref());
    
    match invoked_skill {
      Some(s) => {
        have_invoked_skill = true;
        parsed_prompt.push_str(s);
        parsed_prompt.push('\n');
        args.next();
      },
      None => { break },
    }
  }

  match have_invoked_skill {
    true => {
      let args: Vec<&str> = args.collect(); // scrap the rest of the args
      parsed_prompt = replace_args(parsed_prompt, args);
    },
    false => {
      parsed_prompt = prompt;
    },
  }

  parsed_prompt
}


/// Implement args replacement on invoked skill
/// source: https://code.claude.com/docs/en/skills#available-string-substitutions
pub fn replace_args<'a>(invoked_skill: String, args: Vec<&str>) -> String {
  let mut result = invoked_skill;
  let mut counter = 0; // args counter

  let pattern_target_0 = "$ARGUMENTS";
  result = result.replace(pattern_target_0, &args.join(" "));

  for arg in args {
    let pattern_target_1 = format!("$ARGUMENTS[{}]", counter);
    let pattern_target_2 = format!("${}", counter);
    result = result
              .replace(&pattern_target_1, arg)
              .replace(&pattern_target_2, arg);
    counter += 1;
  }

  result
}