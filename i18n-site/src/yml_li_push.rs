use std::path::Path;

use aok::Result;

fn _push(
  yml: &Path,
  line: &str,
  push: impl Fn(&mut Vec<String>, String, String) -> String,
) -> Result<Vec<String>> {
  if !yml.exists() {
    ifs::wtxt(yml, format!("- {line}"))?;
    return Ok(vec![line.to_owned()]);
  }
  let txt = ifs::rtxt(yml)?;
  let mut li: Vec<String> = serde_yaml::from_str(&txt)?;
  let string = line.to_owned();
  if !li.contains(&string) {
    let txt = push(&mut li, string, txt);
    ifs::wtxt(yml, txt)?;
  }
  Ok(li)
}

pub fn yml_li_push(yml: &Path, line: &str) -> Result<Vec<String>> {
  _push(yml, line, |li, line, mut txt| {
    if !txt.ends_with("\n") {
      txt.push('\n')
    }
    txt.push_str("- ");
    txt.push_str(&line);
    li.push(line);
    txt
  })
}

pub fn yml_li_lpush(yml: &Path, line: &str) -> Result<Vec<String>> {
  _push(yml, line, |li, line, txt| {
    let mut prefix = format!("- {line}");
    if !txt.starts_with("\n") {
      prefix.push('\n')
    }
    li.insert(0, line);
    format!("{prefix}{txt}")
  })
}
