use std::path::Path;

use aok::Result;
use globset::GlobSet;
use gxhash::{HashMap, HashMapExt, HashSet};
use ifs::unix_path;
use lang::{Lang, LANG_CODE};
use md_title::md_title;
use walkdir::WalkDir;
use ytree::sitemap::Sitemap;

use super::{md_htm::article, LangRelTitleHtm, MdHtm, Rss, README_MD};

pub const TOC: &str = "TOC";

pub static EMPTY: String = String::new();

pub async fn scan(
  host: &str,
  root: &Path,
  lang_li: &[Lang],
  changed: &HashSet<String>,
  ignore: &GlobSet,
  lang_foot: &HashMap<Lang, String>,
  sitemap: &mut Sitemap,
  rss: &mut Rss,
) -> Result<Option<LangRelTitleHtm>> {
  let (to_insert, to_remove) = {
    let mut regen_readme = false;
    let mut to_insert = vec![];
    let mut to_remove = rss.exist.clone();
    let mut toc_dir = vec![];

    for lang in lang_li {
      let lang = *lang;
      let lang_en = LANG_CODE[lang as usize];
      let foot = lang_foot.get(&lang).unwrap_or(&EMPTY);
      let dir = root.join(lang_en);
      'out: for entry in WalkDir::new(&dir).into_iter().filter_entry(dot_hide::not) {
        if let Ok(entry) = xerr::ok!(entry) {
          let full_path = entry.path();
          if let Ok(path) = full_path.strip_prefix(&dir)
            && let Some(path_rel) = path.to_str()
          {
            let path_rel = unix_path(path_rel);

            if let Ok(meta) = entry.metadata() {
              let file_type = meta.file_type();
              if file_type.is_dir() {
                let toc = full_path.join(TOC);
                if toc.exists() {
                  toc_dir.push(path_rel.to_owned());

                  let toc_li = rtoc::r(&toc)?.into_iter().collect::<Vec<_>>();

                  for lang in lang_li {
                    let lang = *lang;
                    let lang_en = LANG_CODE[lang as usize];
                    let mut toc_change = false;
                    let mut name_title = HashMap::new();
                    let mut insert = Vec::with_capacity(toc_li.len());
                    for name in &toc_li {
                      let is_readme = name == README_MD;
                      let rel = format!("{path_rel}/{name}");
                      let lang_rel = format!("{lang_en}/{rel}");
                      let is_exist = to_remove.remove(lang, &rel);
                      if is_exist && !changed.contains(&lang_rel) {
                        continue;
                      }
                      if is_readme {
                        toc_change = true;
                        regen_readme = true;
                        continue;
                      }
                      let fp = root.join(&lang_rel);
                      if fp.exists() {
                        let mut md_htm = MdHtm::load(root.join(fp))?;
                        name_title.insert(name, md_htm.title().to_owned());
                        if let Some(htm) = md_htm.htm() {
                          toc_change = true;
                          let title = md_htm.title();
                          rss.push(lang, &rel, title, &htm);
                          insert.push((lang, rel.clone(), title.to_owned(), article(htm)));
                        }
                      }
                    }

                    if toc_change {
                      let mut nav = String::new();
                      for name in &toc_li {
                        if name == README_MD {
                          continue;
                        }
                        let rel = format!("{path_rel}/{name}");
                        let lang_rel = format!("{lang_en}/{rel}");
                        if let Some(title) = name_title.remove(name) {
                          nav += &format!(r#"<a href="/{lang_rel}">{title}</a>"#);
                        } else {
                          let fp = root.join(&lang_rel);
                          if fp.exists() {
                            let mut md_htm = MdHtm::load(fp)?;
                            let title = md_htm.title().to_owned();
                            if md_htm.only_title {
                              if !title.is_empty() {
                                nav += &format!(r#"<p>{title}</p>"#);
                              }
                              continue;
                            }
                            nav += &format!(r#"<a href="/{lang_rel}">{title}</a>"#);
                          }
                        };
                      }

                      let readme_rel = format!("{path_rel}/{README_MD}");
                      let fp = root.join(lang_en).join(&readme_rel);

                      let foot = lang_foot.get(&lang).unwrap_or(&EMPTY);
                      let (title, htm) = if fp.exists() {
                        let mut t = MdHtm::load(fp)?;
                        let h = t.htm();

                        let title = t.title();

                        for i in insert.iter_mut() {
                          i.3 = format!(
                            r#"<main>{}<nav><a href="/">{host}</a><a href="/{lang_en}/{path_rel}">{title}</a></nav>{foot}</main>"#,
                            i.3
                          );
                        }

                        let htm = if let Some(h) = h {
                          rss.push(lang, &readme_rel, title, &h);
                          let h = article(h);
                          let nav = format!(
                            r#"<nav><h1><a href="/{lang_en}/{path_rel}">{title}</a></h1>{nav}</nav>"#
                          );
                          format!("<main>{h}{foot}</main>{nav}",)
                        } else {
                          format!("<main><nav><h1>{title}</h1>{nav}</nav>{foot}</main>")
                        };
                        (t.title().to_owned(), htm)
                      } else {
                        ("".into(), format!(r#"<main><nav>{nav}</nav>{foot}</main>"#))
                      };
                      to_insert.extend(insert);
                      to_insert.push((lang, readme_rel, title, htm));
                    }
                  }
                }
              } else if file_type.is_file() {
                for i in &toc_dir {
                  if path_rel.starts_with(i) && path_rel[i.len()..].starts_with('/') {
                    continue 'out;
                  }
                }
                if let Some(ext) = path.extension() {
                  if ext == "md" {
                    // if path_rel.starts_with("i18") {
                    //   println!("{lang:?} {path_rel} {toc_dir:?}");
                    // }
                    let fp = format!("{lang_en}/{path_rel}");
                    if ignore.is_match(format!("/{fp}")) {
                      continue;
                    }
                    if to_remove.remove(lang, &path_rel) && !changed.contains(&fp) {
                      continue;
                    }
                    if path_rel == README_MD {
                      regen_readme = true;
                      continue;
                    }
                    let mut md_htm = MdHtm::load(root.join(fp))?;
                    if let Some(htm) = md_htm.htm() {
                      let title = md_htm.title();
                      rss.push(lang, &path_rel, title, &htm);
                      let htm = format!("<main>{}{foot}</main>", article(htm));
                      to_remove.remove(lang, &path_rel);
                      to_insert.push((lang, path_rel, title.into(), htm));
                    }
                  }
                }
              }
            }
          };
        }
      }
    }
    if regen_readme {
      for lang in lang_li {
        let lang = *lang;
        let lang_en = LANG_CODE[lang as usize];
        let fp = root.join(lang_en).join(README_MD);
        if fp.exists() {
          let mut md_htm = MdHtm::load(root.join(fp))?;
          let htm = md_htm.htm();
          let title = md_htm.title();
          let htm = if let Some(htm) = htm {
            rss.push(lang, README_MD, title, &htm);
            article(htm)
          } else {
            "".into()
          };
          let nav: Vec<String> = toc_dir
            .iter()
            .filter_map(|rel| {
              let toc = root.join(lang_en).join(rel).join(README_MD);
              if toc.exists() {
                if let Ok(toc) = std::fs::read_to_string(toc) {
                  let title = md_title(&toc);
                  return Some(format!(r#"<a href="/{lang_en}/{rel}">{title}</a>"#));
                }
              }
              None
            })
            .collect::<Vec<_>>();

          let foot = lang_foot.get(&lang).unwrap_or(&EMPTY);
          let htm = format!("<main>{htm}{foot}</main>");
          to_insert.push((
            lang,
            README_MD.into(),
            title.into(),
            if nav.is_empty() {
              htm
            } else {
              format!("{htm}<nav>{}</nav>", nav.join(""))
            },
          ));
        }
      }
    }
    (to_insert, to_remove.set())
  };

  if to_remove.is_empty() && to_insert.is_empty() {
    return Ok(None);
  }

  for (lang, rel) in to_remove {
    rss.exist.remove(lang, rel);
  }

  for (lang, rel, ..) in &to_insert {
    rss.exist.insert(*lang, rel);
  }

  Ok(Some(to_insert))
}
