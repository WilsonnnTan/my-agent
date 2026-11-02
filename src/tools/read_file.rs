use std::{
  fs::OpenOptions, io::Read,
};
use serde::Deserialize;
use anyhow::Result;
use super::Tool;

#[derive(Deserialize)]
pub struct ReadFile<'a> {
  file_path: &'a str,
}

impl<'a> Tool for ReadFile<'a> {
  fn run(&self) -> Result<String> {
    let mut file = OpenOptions::new()
                    .read(true)
                    .open(self.file_path)?;

    let mut data: String = String::new();
    let _ = file.read_to_string(&mut data);

    return Ok(data);
  }
}