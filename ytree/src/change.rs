use std::path::Path;

use bincode::{Decode, Encode};
pub use walkdir::WalkDir;

#[derive(Encode, Decode)]
pub struct LenTs {
  pub ts: u64,
  pub len: u64,
}

#[derive(Encode, Decode)]
pub struct Meta {
  pub len_ts: LenTs,
  pub hash: u128,
}

pub struct Change {
  pub li: Vec<(String, LenTs)>,
}
// let yml_fp = yml_fp.as_ref();
// let r = vec![];
// if yml_fp.exists() {
//   let file = std::io::BufReader::new(std::fs::File::open(yml_fp)?);
//   let mut li = vec![];
//   for line in file.lines() {
//     if let Ok(line) = line {
//       let line = line.trim_end();
//       if let Some(i) = line.chars().next() {
//         if "<>#".contains(i) {
//           continue;
//         } else {
//           li.push(line.to_owned());
//         }
//       } else {
//         continue;
//       }
//     }
//   }
//   let yml = li.join("\n");
//
//   if let Ok(li) = xerr::ok!(serde_yaml::from_str::<crate::Li>(&yml)) {
//     for i in li.iter() {
//       dbg!(i);
//     }
//   }
// }

impl Change {
  pub fn load(public: impl AsRef<Path>) -> std::io::Result<Self> {
    let mut li = vec![];
    for entry in WalkDir::new(&public).into_iter() {
      if let Ok(entry) = entry
        && entry.file_type().is_file()
      {
        let path = entry.path();
        if let Ok(meta) = std::fs::metadata(entry.path())
          && let Ok(ts) = meta.modified()
          && let Ok(rel) = entry.path().strip_prefix(&public)
        {
          li.push((
            rel.to_string_lossy().to_string(),
            LenTs {
              ts: ts.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
              len: meta.len(),
            },
          ));
        }
      }
    }

    Ok(Self { li })
  }
}
