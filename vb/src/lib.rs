use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("vbyte decode error : {0}")]
  VbyteDecode(String),
}

pub fn e(vs: impl AsRef<[u64]>) -> Vec<u8> {
  vbyte::compress_list(vs.as_ref())
}

pub fn d(vs: impl AsRef<[u8]>) -> Result<Vec<u64>, Error> {
  match vbyte::decompress_list(vs.as_ref()) {
    Ok(r) => Ok(r),
    Err(err) => Err(Error::VbyteDecode(err.to_string())),
  }
}

pub fn diffe(li: impl AsRef<[u64]>) -> Vec<u8> {
  let li = li.as_ref();
  if li.len() >= 2 {
    let mut li = Vec::from(li);
    for i in (1..li.len()).rev() {
      li[i] -= li[i - 1];
    }
    e(li)
  } else {
    e(li)
  }
}

pub fn diffd(vs: impl AsRef<[u8]>) -> Result<Vec<u64>, Error> {
  let mut li = d(vs)?;
  if li.len() >= 2 {
    for i in 1..li.len() {
      li[i] += li[i - 1];
    }
  }

  Ok(li)
}
