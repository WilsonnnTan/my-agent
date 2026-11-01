pub mod read_file;

pub trait Run {
  fn run(file_path: &str);
}