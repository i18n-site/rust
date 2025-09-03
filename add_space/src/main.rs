use std::{
  fs,
  io::{self, Read},
};

use add_space::add_space;
use clap::Parser;

#[derive(Parser)]
#[command(name = "add_space", author, version, about, long_about = None)]
struct Cli {
  /// The path to the file to process, or stdin if not provided
  path: Option<String>,

  /// Write the output back to the file
  #[arg(short, long)]
  write: bool,
}

fn main() -> io::Result<()> {
  let cli = Cli::parse();

  let (content, from_stdin) = if let Some(path) = &cli.path {
    (fs::read_to_string(path)?, false)
  } else {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    (buffer, true)
  };

  let new_content = content
    .lines()
    .map(add_space)
    .collect::<Vec<_>>()
    .join("\n");

  if cli.write {
    if from_stdin {
      eprintln!("Error: cannot use --write with stdin.");
      std::process::exit(1);
    }
    if let Some(path) = &cli.path {
      fs::write(path, new_content)?;
      println!("File {} has been updated.", path);
    }
  } else {
    print!("{}", new_content);
  }

  Ok(())
}
