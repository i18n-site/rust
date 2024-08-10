use std::{
  fs::File,
  io::{Read, Write},
  path::Path,
};

use tar::Builder;
use zstd::stream::Encoder;

pub struct W {
  tzst: Builder<Vec<u8>>,
}

impl Default for W {
  fn default() -> Self {
    Self::new()
  }
}

impl W {
  pub fn new() -> Self {
    Self {
      tzst: Builder::new(vec![]),
    }
  }

  pub fn add_path_li<A>(
    &mut self,
    root: impl AsRef<Path>,
    mut rel_li: Vec<A>,
  ) -> std::io::Result<()>
  where
    A: AsRef<str>,
  {
    let root = root.as_ref();
    rel_li.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

    for relative_path in rel_li {
      let relative_path = relative_path.as_ref();
      let full_path = root.join(relative_path);

      if full_path.is_file() {
        let mut file = File::open(&full_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut header = tar::Header::new_gnu();
        header.set_size(buffer.len() as u64);
        header.set_cksum();

        self
          .tzst
          .append_data(&mut header, relative_path, &*buffer)?;
      } else if full_path.is_dir() {
        let mut header = tar::Header::new_gnu();
        header.set_size(0);
        header.set_mode(0o755);
        header.set_cksum();

        self
          .tzst
          .append_data(&mut header, relative_path, &[] as &[u8])?;
      }
    }

    Ok(())
  }

  pub fn add_bin(&mut self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> std::io::Result<()> {
    let relative_path = rel.as_ref();
    let bin_data = bin.as_ref();

    let mut header = tar::Header::new_gnu();
    header.set_size(bin_data.len() as u64);
    header.set_cksum();

    self
      .tzst
      .append_data(&mut header, relative_path, bin_data)?;

    Ok(())
  }

  pub fn end(&mut self) -> std::io::Result<Vec<u8>> {
    // Take ownership of self.tzst and replace it with a new Builder
    let tar_builder = std::mem::replace(&mut self.tzst, Builder::new(vec![]));

    let tar_data = tar_builder.into_inner()?;

    let mut encoder = Encoder::new(Vec::new(), 0)?;
    encoder.write_all(&tar_data)?;
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
  }
}
