use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
  /// The pattern to look for
  pattern: String,
  path: PathBuf,
}

fn main() {
  let args = Cli::parse();
  let content = read_to_string(&args.path).expect("An error occured readding file");

  for (index, line) in content.lines().enumerate() {
    if line.contains(&args.pattern) {
      println!("{}: {line}", index + 1)
    }
  }
}
