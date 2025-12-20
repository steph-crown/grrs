use clap::{Error, Parser};
use std::{
  fs::{File, read_to_string},
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

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
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

fn process_args_v2(args: &Cli) -> Result<(), CustomError> {
  let file = File::open(&args.path).map_err(|err| {
    CustomError(format!(
      "Error reading file at path `{:?}`: {}",
      args.path, err
    ))
  })?;

  // let file_content = match file {
  //   Ok(content) => content,
  //   Err(err) => return Err(err.into()),
  // };

  let reader = BufReader::new(&file);

  for (index, line) in reader.lines().enumerate() {
    let line_content = line.expect("Error reading line");

    if line_content.contains(&args.pattern) {
      println!("{}: {}", index + 1, line_content)
    }
  }

  Ok(())
}
