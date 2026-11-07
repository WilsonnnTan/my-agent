mod validator;
pub mod skill_loader;

use serde::Deserialize;
use std::{
  collections::HashMap, path::PathBuf,
};


/// Advertise skill by get the Metadata (name and description) of all skills
/// source: https://agentskills.io/specification#progressive-disclosure
pub fn skills_advertisement(skills: &HashMap<String, Skill>) -> String{
  let mut advertisement = String::from("You have access to the following skills:\n\n");

  for (_, value) in skills {
    let metadata = value.get_metadata();
    advertisement.push_str(&metadata);
    advertisement.push('\n');
  }

  advertisement.push_str("If a skill matches the user's request, call the InvokeSkill tool with its name and follow the instructions it returns.");

  advertisement
}


/// A parsed skill definition that deserialized with serde_yaml.
// `SKILL.md` file must contain YAML frontmatter followed by Markdown content
#[derive(Deserialize)]
pub struct Skill {
  /// The name of the skill - required
  pub name: String,
  /// The description of the skill - required
  pub description: String,
  /// The License name or reference to a bundled license file. - optional
  pub license: Option<String>,
  /// Indicates environment requirements (intended product, system packages, network access, etc.) - optional
  pub compatibility: Option<String>,
  /// The map from string keys to string values - optional
  pub metadata: Option<HashMap<String, String>>,
  /// The content of the skill
  pub content: Option<String>,
  /// The file path to the skill file
  pub file_path: Option<PathBuf>,
}

impl Skill {
  fn set_content(&mut self, content: String) {
    self.content = Some(content);
  }

  fn set_file_path(&mut self, file_path: PathBuf) {
    self.file_path = Some(file_path);
  }

  /// This is a method to validate the format of Skill
  /// source: https://agentskills.io/specification
  fn validate_format(&self) {
    todo!()
  }

  /// This is a method to get the Metadata (name, description, and path) of the skill
  /// source: https://agentskills.io/specification#progressive-disclosure
  pub(crate) fn get_metadata(&self) -> String {
    match &self.file_path {
      Some(file_path) => format!("- {}: {} (located at {})", self.name, self.description, file_path.display()),
      None => format!("- {}: {}", self.name, self.description),
    }
  }

  /// This is a method to get skill content
  /// with name, path, and replaced args in it
  pub(crate) fn get_skill_content(&self, args: &[&str]) -> String {
    let mut skill_content = String::new();

    if let Some(file_path) = &self.file_path {
      let name_with_file_path = format!("Skill: {} (located at {})", &self.name, &file_path.to_string_lossy());
      skill_content.push_str(&name_with_file_path);      
    }
    if let Some(content) = &self.content {
      skill_content.push_str(&Self::replace_args(content, args));
    }

    skill_content
  }

  /// Implement args replacement on invoked skill
  /// source: https://code.claude.com/docs/en/skills#available-string-substitutions
  pub(crate) fn replace_args(content: &str, args: &[&str]) -> String {
    let mut result: String = String::new();

    let mut content = content;

    let regex_pattern = regex::Regex::new(r#"^\$ARGUMENTS\[(\d+)\]|^\$(\d+)|^\$ARGUMENTS"#).unwrap();

    while let Some(start_idx) = content.find("$") {
      result.push_str(&content[..start_idx]);
      content = &content[start_idx..]; // slice to take unprocessed string

      if let Some(caps) = regex_pattern.captures(content) {
        let end = caps.get(0).unwrap().end();

        match (caps.get(1), caps.get(2)) {
          // $ARGUMENTS[m] or $m
          (Some(m), _) | (None, Some(m)) => {
            let idx: usize = m.as_str().parse().unwrap_or(usize::MAX);
            result.push_str(args.get(idx).unwrap_or(&""));
          },
          // $ARGUMENTS
          (None, None) => {
            let concated_args: String = args.join(" ");
            result.push_str(&concated_args);
          },
        }

        content = &content[end..];
      } else {
        // Covering case when there is '$' but doesnt match the regex_pattern
        // This will help to avoid infinite loop
        result.push('$');
        content = &content[1..];
      }
    }

    result.push_str(content);

    result
  }
}
