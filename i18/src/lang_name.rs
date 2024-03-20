use std::{collections::HashMap, io::Write};

use aok::Result;
use lang::Lang;

use crate::NAME;

pub async fn lang_name_li(
  id_li: impl AsRef<[u16]>,
  lang_str: &HashMap<Lang, String>,
) -> Result<Vec<String>> {
  let id_li = id_li.as_ref();
  let mut r = Vec::with_capacity(id_li.len());
  let mut id_name = Vec::new();

  for i in id_li.iter() {
    r.push(if let Ok::<Lang, _>(lang) = (*i).try_into() {
      if let Some(name) = lang_str.get(&lang) {
        name.clone()
      } else {
        lang.code().into()
      }
    } else {
      #[allow(clippy::never_loop)]
      loop {
        if id_name.is_empty() {
          macro_rules! id_name {
            ($txt:expr) => {
              id_name = $txt.split('\n').map(|i| i.into()).collect::<Vec<String>>();
            };
          }

          let conf = ifs::confdir().join(NAME).join("code.txt");

          if let Ok(txt) = std::fs::read_to_string(&conf) {
            id_name!(txt);

            if (id_name.len() as u16) > *i {
              break;
            }
          }

          let url = "https://atomgit.com/i18n-site/rust/raw/dev/lang/code.txt";
          let txt: String = ireq::get(url).await?;
          ifs::w(conf)?.write_all(txt.as_bytes())?;
          id_name!(txt);
        }
        break;
      }
      id_name[*i as usize].clone()
    })
  }
  Ok(r)
}
