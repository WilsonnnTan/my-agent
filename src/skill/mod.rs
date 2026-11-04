pub mod validator;

use serde::Deserialize;
use std::collections::HashMap;

pub fn skill_search() {
  // search for claude skill folder in current dir, such as:
  // - `.claude/skills/`
  // - `.agents/skills`
  //
  // call skill_yaml_parser
  // deserealize into struct Skill
  // call validator
  todo!()
}

pub fn skill_yaml_parser() {
  // make sure there is frontmatter by creating a yaml parser
  todo!()
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
}

impl Skill {}