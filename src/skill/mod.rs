pub mod validator;

use serde::Deserialize;
use std::{
  collections::HashMap, fs::{self, OpenOptions}, io::Read, path::PathBuf,
};
use anyhow::{
  Result,
  anyhow,
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

  advertisement
}

/// Search and load skills into memory
pub fn load_skills() -> Result<HashMap<String, Skill>> {
  // this function will help loading skills, and it will merge multiple skill_dir instead of picking one of them
  // search for skill in current dir, such as:
  // - `.agents/skills`
  // - `.claude/skills/`
  //
  // call parse_skill (deserealize into struct Skill)
  // call validator (to validate each field format)
  let mut map: HashMap<String, Skill> = HashMap::new();

  // TODO: update to not only read the current project skills
  let skill_dirs: [PathBuf; 2] = [
    PathBuf::from(".agents/skills/"),
    PathBuf::from(".claude/skills/")
  ];

  for skill_dir in skill_dirs {
    if !skill_dir.is_dir() { continue; }

    for entry in fs::read_dir(skill_dir)? {
      let dir = entry?.path();
      let skill_file = dir.join("SKILL.md");

      if !skill_file.is_file() { continue; }

      // Read skill content
      let mut f = OpenOptions::new()
                .read(true)
                .open(&skill_file)?;

      let mut skill_content = String::new();
      f.read_to_string(&mut skill_content)?;
      
      // Replace \r\n to \n to handle CRLF (Carriage Return Line Feed) on windows
      #[cfg(windows)]
      let skill_content = skill_content.replace("\r\n", "\n");

      // Parse skill content
      let skill = parse_skill(skill_file, &skill_content)?;

      map.insert(skill.name.clone(), skill);
    }
  }

  Ok(map)
}


/// Deserealize skill content into `SKILL` struct
pub fn parse_skill<'a>(skill_file: PathBuf, file_content: &'a str) -> Result<Skill> {
  let convention_error = || { anyhow!(
                          "{} doesn't have proper skill format.
  check skill file's convention on: https://agentskills.io/specification",
                          skill_file.display()
                        )};

  let file_content = file_content
                      .strip_prefix("---\n")
                      .ok_or_else(convention_error)?;
  
  let end = file_content
                .find("\n---")
                .ok_or_else(convention_error)?;
  
  let skill_frontmatter_content = &file_content[..end];

  let mut skill: Skill = serde_yaml::from_str(skill_frontmatter_content)?;

  skill.set_content(String::from(&file_content[end+4..]));
  skill.set_file_path(skill_file);

  Ok(skill)
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
  pub fn set_content(&mut self, content: String) {
    self.content = Some(content);
  }

  pub fn set_file_path(&mut self, file_path: PathBuf) {
    self.file_path = Some(file_path);
  }

  /// This is a method to validate the format of Skill
  /// source: https://agentskills.io/specification
  pub fn validate_format(&self) {
    todo!()
  }

  /// This is a method to get the Metadata (name, description, and path) of the skill
  /// source: https://agentskills.io/specification#progressive-disclosure
  pub fn get_metadata(&self) -> String {
    match &self.file_path {
      Some(file_path) => format!("- {}: {} (located at {})", self.name, self.description, file_path.display()),
      None => format!("- {}: {}", self.name, self.description),
    }
  }

  /// Get skill content with skill name and path in it
  pub fn get_skill_content(&self) -> String {
    let mut skill_content = String::new();

    if let Some(file_path) = &self.file_path {
      let name_with_file_path = format!("Skill: {} (located at {})", &self.name, &file_path.to_string_lossy());
      skill_content.push_str(&name_with_file_path);      
    }
    if let Some(content) = &self.content {
      skill_content.push_str(content);
    }

    skill_content
  }
}