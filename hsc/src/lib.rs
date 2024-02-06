#![feature(macro_metavar_expr)]

use std::path::Path;

use aok::{Error, Result, OK};
use blake3::Hasher;
use ed25519_dalek::{Signer, SigningKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use tokio::{
  fs::{write, File},
  io::AsyncReadExt,
};
// use trt::join;

pub async fn key(key: impl AsRef<Path>, create: bool) -> Result<SigningKey> {
  let key = key.as_ref();
  match File::open(key).await {
    Ok(mut k) => {
      let mut signing_key_bytes: [u8; SECRET_KEY_LENGTH] = Default::default();
      k.read_exact(&mut signing_key_bytes).await?;
      Ok::<_, Error>(SigningKey::from_bytes(&signing_key_bytes))
    }
    Err(err) => {
      if create && err.kind() == tokio::io::ErrorKind::NotFound {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);

        let key_str = key.as_os_str().to_string_lossy();
        let pk_fp = if key_str.ends_with(".sk") {
          key_str[..key_str.len() - 2].to_owned() + "pk"
        } else {
          (key_str + ".pk").into()
        };
        write(pk_fp, signing_key.verifying_key().as_bytes()).await?;
        write(key, signing_key.as_bytes()).await?;
        Ok(signing_key)
      } else {
        Err(err.into())
      }
    }
  }
}

pub async fn hsc(fp: impl AsRef<Path>, key: SigningKey) -> Result<()> {
  let fp = fp.as_ref();
  let hash: Hasher = ifs::hash(fp).await?;
  let hash = hash.finalize();
  let sign = key.sign(hash.as_bytes()).to_bytes();
  // let mut b3 = fp.to_owned().into_os_string();
  // b3.push(".b3");
  let mut hsc = fp.to_owned().into_os_string();
  hsc.push(".hsc");
  write(hsc, sign).await?;
  /*
  use ed25519_dalek::Verifier;
  let verifying_key = key.verifying_key();
  let r = verifying_key.verify(&hash, &Signature::from_bytes(&sign));
  dbg!(r);
  */
  OK
}
