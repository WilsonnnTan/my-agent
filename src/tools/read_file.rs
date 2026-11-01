use std::{
  fs::OpenOptions, io::Read,
};

use super::Run;

pub struct ReadFile;

impl Run for ReadFile {
  fn run(file_path: &str) {
    if let Ok(mut file) = OpenOptions::new()
                                  .read(true)
                                  .open(file_path)
    {
      let mut data: String = String::new();
      let _ = file.read_to_string(&mut data);
      println!("{}", data);
    }
  }
}