use std::{
  collections::HashMap, fs::{self, OpenOptions}, io::Read, path::PathBuf,
};
use anyhow::{
  Result,
  anyhow,
};
use super::Skill;


pub struct SkillLoader;

/// Implementation of Skill's Loader
impl SkillLoader {
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
        let skill = Self::parse_skill(skill_file, &skill_content)?;

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
}