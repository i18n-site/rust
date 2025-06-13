use std::io::{self, Read};

pub struct LineIter<R: Read> {
  reader: R,
  buffer: Vec<u8>,
  position: usize,
  last_line_returned: bool,
}

impl<R: Read> LineIter<R> {
  pub fn new(reader: R) -> Self {
    LineIter {
      reader,
      buffer: Vec::new(),
      position: 0,
      last_line_returned: false,
    }
  }
}

impl<R: Read> Iterator for LineIter<R> {
  type Item = io::Result<String>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.last_line_returned {
      return None;
    }

    let mut line = Vec::new();
    loop {
      if self.position >= self.buffer.len() {
        self.buffer.clear();
        self.position = 0;
        let mut temp_buf = [0; 1024];
        match self.reader.read(&mut temp_buf) {
          Ok(0) => break,
          Ok(n) => self.buffer.extend_from_slice(&temp_buf[..n]),
          Err(e) => return Some(Err(e)),
        }
      }

      while self.position < self.buffer.len() {
        let byte = self.buffer[self.position];
        self.position += 1;

        if byte == b'\n' {
          return Some(Ok(String::from_utf8_lossy(&line).to_string()));
        } else if byte == b'\r' {
          if self.position < self.buffer.len() && self.buffer[self.position] == b'\n' {
            self.position += 1;
          }
          return Some(Ok(String::from_utf8_lossy(&line).to_string()));
        } else {
          line.push(byte);
        }
      }
    }

    if !line.is_empty() {
      self.last_line_returned = true;
      return Some(Ok(String::from_utf8_lossy(&line).to_string()));
    }

    None
  }
}
