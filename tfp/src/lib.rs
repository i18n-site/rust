use std::{
  fs::File,
  io::{BufReader, Read, Write},
  path::Path,
};

pub fn tfp(fp: impl AsRef<Path>) -> std::io::Result<String> {
  let file = File::open(&fp)?;
  let mut reader = BufReader::new(file);
  let mut buffer = [0; 1]; // Read one byte at a time

  let mut change = false;
  let mut li = Vec::new();
  let mut line = Vec::new();

  loop {
    let bytes_read = reader.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }

    let c = buffer[0];
    match c {
      b'\n' => {
        let t = String::from_utf8_lossy(&line);
        let trimmed_line = t.trim_end();
        if trimmed_line.len() != t.len() {
          change = true;
        }
        li.push(trimmed_line.to_owned());
        line.clear();
      }
      b'\r' => {
        change = true;
        let t = String::from_utf8_lossy(&line);
        let trimmed_line = t.trim_end();
        li.push(trimmed_line.to_owned());
        line.clear();
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read > 0 {
          let c = buffer[0];
          if c != b'\n' {
            // Skip the next \n
            if c == b'\r' {
              li.push("".into());
            } else {
              line.push(c);
            }
          }
        }
      }
      _ => {
        line.push(c);
      }
    }
  }

  if !line.is_empty() {
    let t = String::from_utf8_lossy(&line);
    let trimmed_line = t.trim_end();
    if trimmed_line.len() != t.len() {
      change = true;
    }
    li.push(trimmed_line.to_owned());
  }

  while let Some(i) = li.last() {
    if i.is_empty() {
      change = true;
      li.pop();
    } else {
      break;
    }
  }

  let txt = li.join("\n");
  if change {
    let mut file = File::create(fp)?;
    file.write_all(txt.as_bytes())?;
  }

  Ok(txt)
}
