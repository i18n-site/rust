use std::io::Write;
pub const MAX_PRINT: usize = 7;

pub fn w<W: Write, S: AsRef<str>>(writer: &mut W, len: usize, iter: impl Iterator<Item = S>) {
  let mut n = 0;
  for i in iter.into_iter() {
    xerr::log!(writeln!(writer, "{}", i.as_ref()));
    if n < MAX_PRINT {
      n += 1;
    } else {
      xerr::log!(writeln!(writer, "  {} more ...", len - n));
      break;
    }
  }
}

pub fn stdout(len: usize, iter: impl Iterator<Item = String>) {
  w(&mut std::io::stdout(), len, iter)
}

pub fn stderr(len: usize, iter: impl Iterator<Item = String>) {
  w(&mut std::io::stderr(), len, iter)
}
