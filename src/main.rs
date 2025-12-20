// use anyhow::Context;
use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::{
  fs::{File, read_to_string},
  io::{BufRead, BufReader},
  path::PathBuf,
};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
  pattern: String,
  path: PathBuf,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  return process_args_v1(&args);
}

fn process_args_v1(args: &Cli) -> Result<()> {
  let content = read_to_string(&args.path)
    .with_context(|| format!("An error occured opening at path {:?}", args.path))?;

  grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

  Ok(())
}

#[test]
fn test_find_matches() {
  let content = String::from("lorem ipsum\ndolor sit amet");
  let pattern = String::from("lorem");

  let mut result = Vec::new();
  grrs::find_matches(&content, &pattern, &mut result);
  println!("{:?}", result);
  assert_eq!(result, b"1: lorem ipsum\n");
}

// uses buffer
// fn process_args_v2(args: &Cli) -> Result<()> {
//   let file = File::open(&args.path)
//     .with_context(|| format!("An error occured opening at path {:?}", args.path))?;

//   let reader = BufReader::new(&file);

//   for (index, line) in reader.lines().enumerate() {
//     let line_content = line.expect("Error reading line");

//     if line_content.contains(&args.pattern) {
//       println!("{}: {}", index + 1, line_content);
//       eprintln!("errorororo")
//     }
//   }

//   Ok(())
// }

/* TESTS */
