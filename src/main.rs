use clap::Parser;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
  /// The pattern to look for
  pattern: String,
  path: PathBuf,
}

fn main() {
  let args = Cli::parse();
  let a: PathBuf = PathBuf::from("sjjs");
  let b = a.to_str();

  println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
}
