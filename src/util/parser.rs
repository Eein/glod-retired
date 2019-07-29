use livesplit_core::run::parser::composite;
use livesplit_core::Run;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct SplitParser;
impl SplitParser {
  pub fn load(&self) -> Run {
    // Load the file.
    let path = PathBuf::from("src/splits/earthbound.lss");
    let file = BufReader::new(File::open(&path).expect("File not found"));

    // We want to load additional files from the file system, like segment icons.
    let load_files = true;

    // Actually parse the file.
    let result = composite::parse(file, Some(path), load_files);
    let parsed = result.expect("Not a valid splits file");

    parsed.run
  }
}