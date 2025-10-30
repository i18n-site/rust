use std::{
  collections::{HashMap, HashSet},
  fs::{self, File},
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
    let lines: Vec<String> = lines.map_while(Result::ok).collect();
    let mut user_token_set_li = vec![];
    let mut user = String::new();
    let mut token_set = HashSet::new();
    let mut line_iter = lines.iter();
    'out: while let Some(line) = line_iter.next() {
      if line.starts_with("token_li:") {
        for line in line_iter {
          let line = line.trim();
          if let Some(line) = line.strip_prefix("#") {
            if line.trim_start().starts_with("-") {
              continue;
            }
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
    let ban = verify_token_set(&user_token_set_li).await?;

    let mut user_token_set_li: HashMap<String, HashSet<String>> =
      HashMap::from_iter(user_token_set_li.into_iter());
    let mut result = Vec::with_capacity(lines.len());
    let mut line_iter = lines.into_iter();

    'out: while let Some(line) = line_iter.next() {
      let is_token_li = line.starts_with("token_li:");
      result.push(line);
      if is_token_li {
        for line in line_iter.by_ref() {
          let line = line.trim_end();
          let l = line.trim_start();
          if let Some(l) = l.strip_prefix("#") {
            result.push(line.into());
            if l.trim_start().starts_with("-") {
              continue;
            }
            let user = l.trim_start();
            if let Some(token_set) = user_token_set_li.remove(user) {
              let mut ok_li = vec![];
              let mut err_li = vec![];
              for token in token_set {
                if ban.contains(&token) {
                  err_li.push(token);
                } else {
                  ok_li.push(token);
                }
              }
              err_li.sort();
              ok_li.sort();
              for i in ok_li {
                result.push(format!("  - {i}"));
              }
              for i in err_li {
                result.push(format!("  # - {i}"));
              }
            }
          } else if l.starts_with("-") {
            continue;
          } else if l.ends_with(":") {
            break 'out;
          } else {
            result.push(line.into());
          }
        }
        break 'out;
      }
    }

    for i in line_iter {
      result.push(i);
    }
    xerr::log!(fs::write(&cli.filepath, result.join("\n")));
  }
  OK
}
