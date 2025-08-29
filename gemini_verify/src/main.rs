use std::{
  collections::HashSet,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

use aok::{OK, Void};
use clap::Parser;

mod verify;
use verify::verify;
mod verify_token_set;
use verify_token_set::verify_token_set;

#[derive(Parser)]
#[command(name = "gemini_verify")]
#[command(about = "verify gemini api token")]
struct Cli {
  filepath: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
pub async fn main() -> Void {
  let cli = Cli::parse();

  if let Ok(lines) = xerr::ok!(read_lines(&cli.filepath)) {
    let mut user_token_set_li = vec![];
    let mut user = String::new();
    let mut token_set = HashSet::new();
    let mut lines = lines.map_while(Result::ok);
    'out: while let Some(line) = lines.next() {
      if line.starts_with("token_li:") {
        for line in lines {
          let line = line.trim();
          if let Some(line) = line.strip_prefix("#") {
            if !token_set.is_empty() {
              user_token_set_li.push((user, token_set));
              token_set = HashSet::new();
            }
            user = line.trim_start().into();
          } else if let Some(line) = line.strip_prefix("-") {
            let token = line.trim_start().to_owned();
            if !token_set.insert(token.clone()) {
              eprintln!("令牌重复 {} : {}", user, token);
            }
          } else if line.ends_with(":") {
            break 'out;
          }
        }
        break 'out;
      }
    }
    if !token_set.is_empty() {
      user_token_set_li.push((user, token_set));
    }
    verify_token_set(user_token_set_li).await?;
  }
  OK
}
