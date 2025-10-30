use std::{fs, io, path::Path};

pub fn cp<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
  let from_path = from.as_ref();
  let to_path = to.as_ref();

  if !from_path.exists() {
    return Err(io::Error::new(
      io::ErrorKind::NotFound,
      "Source file not found",
    ));
  }

  if let Some(to_dir) = to_path.parent()
    && !to_dir.exists()
  {
    fs::create_dir_all(to_dir)?;
  }

  fs::copy(from, to)?;

  Ok(())
}

pub fn cp_rel<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>, P4: AsRef<Path>>(
  from_dir: P1,
  from_rel: P2,
  to_dir: P3,
  to_rel: P4,
) -> io::Result<()> {
  let from_dir = from_dir.as_ref();
  let to_dir = to_dir.as_ref();
  cp(from_dir.join(from_rel), to_dir.join(to_rel))
}
