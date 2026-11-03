use serde_json::Value;

use super::read_file::ReadFile;
use std::sync::LazyLock;

/// Make sure spec() implementation uses unwrap
/// so the program doesn't run if there is error on tool's spec
pub trait Spec {
  fn spec() -> Value;
}

/// `TOOLS` contains the list of tools spec exist in this project
pub static TOOLS_SPEC: LazyLock<[Value; 1]> = LazyLock::new(|| {
  [ReadFile::spec()]
});