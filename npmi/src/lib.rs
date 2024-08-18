#[derive(Debug)]
pub struct Pkg {
  pub name: String,
  pub ver: Option<String>,
}

impl Pkg {
  pub fn new(name_ver: impl AsRef<str>) -> Self {
    let name_ver = name_ver.as_ref();
    let name;
    let ver;
    loop {
      if name_ver.len() > 1 {
        if let Some(mut p) = name_ver[1..].find('@') {
          p += 1;
          name = name_ver[..p].into();
          ver = Some(name_ver[p + 1..].into());
          break;
        }
      }

      name = name_ver.into();
      ver = None;
      break;
    }

    Self { name, ver }
  }
}

pub fn npmi() {}
