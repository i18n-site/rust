use std::{io, path::Path};

use filetime::{FileTime, set_file_mtime};
use num_traits::cast::AsPrimitive;

pub fn set_mtime(fp: &Path, ts: impl AsPrimitive<i64>) -> io::Result<()> {
  let mtime = FileTime::from_unix_time(ts.as_(), 0);
  set_file_mtime(fp, mtime)?;
  Ok(())
}
