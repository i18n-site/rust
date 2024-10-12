use std::{
  io::{self, BufReader, Cursor, Read},
  pin::Pin,
};

use tar::{Entries, Entry};
use tracing::error;
use zstd::stream::Decoder;

#[derive(Debug)]
pub struct File {
  pub path: String,
  pub bin: Vec<u8>,
}

pub type Reader<'a> = BufReader<Cursor<&'a [u8]>>;

pub fn to_file(entry: io::Result<Entry<Decoder<'_, Reader<'_>>>>) -> Option<File> {
  match entry {
    Ok(mut entry) => {
      let path = match entry.path() {
        Ok(p) => p.to_string_lossy().into_owned(),
        Err(e) => {
          error!("{}", e);
          return None;
        }
      };
      let mut bin = Vec::new();
      match entry.read_to_end(&mut bin) {
        Ok(_) => Some(File { path, bin }),
        Err(e) => {
          error!("{}: {}", path, e);
          None
        }
      }
    }
    Err(e) => {
      error!("{}", e);
      None
    }
  }
}

pub type Archive<'a> = tar::Archive<Decoder<'a, BufReader<Cursor<&'a [u8]>>>>;
pub type ZstDecode<'a> = zstd::Decoder<'a, BufReader<std::io::Cursor<&'a [u8]>>>;

pub struct Tzst<'a> {
  _archive: Pin<Box<Archive<'a>>>,
  entries: Entries<'a, ZstDecode<'a>>,
}

impl<'a> Tzst<'a> {
  pub fn load(data: &'a [u8]) -> io::Result<Self> {
    let cursor = Cursor::new(data);
    let decoder = Decoder::new(cursor)?;
    let archive = tar::Archive::new(decoder);
    let mut _archive = Pin::new(Box::new(archive));
    let archive_ref: &'a mut Archive<'a> =
      unsafe { &mut *(_archive.as_mut().get_unchecked_mut() as *mut Archive<'a>) };

    let entries = archive_ref.entries()?;

    Ok(Tzst { _archive, entries })
  }
}

impl Iterator for Tzst<'_> {
  type Item = File;

  fn next(&mut self) -> Option<Self::Item> {
    for next in self.entries.by_ref() {
      if let Some(file) = to_file(next) {
        return Some(file);
      }
    }
    None
  }
}

pub fn r(data: &[u8]) -> io::Result<Tzst<'_>> {
  Tzst::load(data)
}
