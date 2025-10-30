use std::io::Read;

pub fn read(mut reader: impl Read) -> (String, bool) {
  let mut buffer = [0; 1]; // Read one byte at a time

  let mut change = false;
  let mut li = Vec::new();
  let mut line = Vec::new();

  while let Ok(bytes_read) = xerr::ok!(reader.read(&mut buffer)) {
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
        if let Ok(bytes_read) = xerr::ok!(reader.read(&mut buffer))
          && bytes_read > 0
        {
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
      b'\0' => {
        // 不能包含 '\0' 因为用这个做缓存的占位符
        change = true;
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
  (txt, change)
}

#[cfg(feature = "fs")]
pub fn fp(fp: impl AsRef<std::path::Path>) -> std::io::Result<String> {
  use std::{
    fs::File,
    io::{BufReader, Write},
  };
  let file = File::open(&fp)?;
  let mut reader = BufReader::new(file);
  let (txt, change) = read(&mut reader);
  if change {
    let mut file = File::create(fp)?;
    file.write_all(txt.as_bytes())?;
  }

  Ok(txt)
}

#[cfg(feature = "str")]
pub fn str(txt: impl AsRef<str>) -> (String, bool) {
  use std::io::Cursor;
  let txt = txt.as_ref();
  read(Cursor::new(txt))
}
