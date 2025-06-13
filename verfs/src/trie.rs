use indexmap::IndexMap;
use li_pos::LiPos;
use serde::{ser::SerializeSeq, Serialize, Serializer};

// 用 indexmap 保证序列化后的顺序不变

pub type RelVer = IndexMap<String, usize>;

#[derive(Debug, Default, Clone)]
pub struct Trie {
  // 文件 - 版本号id
  pub file: RelVer,
  // 子目录
  pub sub: IndexMap<String, Trie>,
}

impl Serialize for Trie {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if self.sub.is_empty() {
      self.file.serialize(serializer)
    } else {
      let mut seq = serializer.serialize_seq(Some(2))?;
      seq.serialize_element(&self.file)?;
      seq.serialize_element(&self.sub)?;
      seq.end()
    }
  }
}

impl Trie {
  pub fn add(&mut self, path: impl Into<String>, ver_id: usize) {
    let path = path.into();
    if let Some(p) = path.find('/')
      && path.len() > p
    {
      let prefix = &path[..p];
      let rel = &path[p + 1..];
      if let Some(t) = self.sub.get_mut(prefix) {
        t.add(rel, ver_id);
      } else {
        let mut trie = Trie::default();
        trie.add(rel, ver_id);
        self.sub.insert(prefix.into(), trie);
      };
    } else {
      self.file.insert(path, ver_id);
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct ExtTrie {
  pub ver_li: LiPos,
  pub tree: IndexMap<String, Trie>,
}

impl Serialize for ExtTrie {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(2))?;
    seq.serialize_element(&self.ver_li.0.join(";"))?;
    seq.serialize_element(&self.tree)?;
    seq.end()
  }
}

impl ExtTrie {
  pub fn add(&mut self, path: impl Into<String>, ver: impl Into<String>) {
    let ver_id = self.ver_id(ver);
    let path = path.into();

    let (rel, ext) = if let Some(p) = path.rfind('.')
      && path.len() > p
    {
      (&path[..p], &path[p + 1..])
    } else {
      (path.as_ref(), "")
    };

    self.tree.entry(ext.into()).or_default().add(rel, ver_id);
  }

  pub fn ver_id(&mut self, ver: impl Into<String>) -> usize {
    self.ver_li.id(ver)
  }
}
