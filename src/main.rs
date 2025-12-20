// use anyhow::Context;
use anyhow::{Context, Result};
use clap::Parser;
use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::PathBuf,
};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
  /// The pattern to look for
  pattern: String,
  path: PathBuf,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  return process_args_v2(&args);
}

// fn process_args_v1(args: &Cli) {
//   let content = read_to_string(&args.path).expect("An error occured readding file");

//   for (index, line) in content.lines().enumerate() {
//     if line.contains(&args.pattern) {
//       println!("{}: {line}", index + 1)
//     }
//   }
// }

fn process_args_v2(args: &Cli) -> Result<()> {
  let file = File::open(&args.path)
    .with_context(|| format!("An error occured opening at path {:?}", args.path))?;

  let reader = BufReader::new(&file);

  for (index, line) in reader.lines().enumerate() {
    let line_content = line.expect("Error reading line");

    if line_content.contains(&args.pattern) {
      println!("{}: {}", index + 1, line_content)
    }
  }

  Ok(())
}
