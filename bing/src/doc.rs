use std::path::Path;

use anyhow::Result;
use tantivy::{
  Index, IndexWriter, TantivyDocument, TantivyError, Term,
  query::QueryParser,
  schema::{Field, Schema},
};
use txtfmt::txtfmt;

use crate::search::Searcher;

#[derive(Debug, Clone, PartialEq)]
pub struct Doc {
  pub id: u64,
  pub ts: u64,
  pub uid: u64,
  pub org_id: u64,
  pub repo_id: u64,
  pub tag_li: Vec<String>,
  pub title: String,
  pub txt: String,
}

pub struct DocFiled {
  pub id: Field,
  pub uid: Field,
  pub org_id: Field,
  pub repo_id: Field,
  pub tag_li: Field,
  pub ts: Field,
  pub title: Field,
  pub txt: Field,
}

impl DocFiled {
  pub fn new(schema: &Schema) -> Self {
    Self {
      id: schema.get_field("id").unwrap(),
      uid: schema.get_field("uid").unwrap(),
      org_id: schema.get_field("org_id").unwrap(),
      repo_id: schema.get_field("repo_id").unwrap(),
      tag_li: schema.get_field("tag_li").unwrap(),
      ts: schema.get_field("ts").unwrap(),
      title: schema.get_field("title").unwrap(),
      txt: schema.get_field("txt").unwrap(),
    }
  }

  pub fn dump(&self, doc: Doc) -> TantivyDocument {
    let mut tdoc = TantivyDocument::new();
    tdoc.add_u64(self.id, doc.id);
    tdoc.add_u64(self.uid, doc.uid);
    tdoc.add_u64(self.org_id, doc.org_id);
    tdoc.add_u64(self.repo_id, doc.repo_id);
    for i in doc.tag_li.iter() {
      let i = i.trim();
      if !i.is_empty() {
        tdoc.add_text(self.tag_li, i);
      }
    }
    tdoc.add_u64(self.ts, doc.ts);
    tdoc.add_text(self.title, doc.title.trim());
    tdoc.add_text(self.txt, txtfmt(doc.txt));
    tdoc
  }
}

#[static_init::dynamic]
pub static FIELD: DocFiled = DocFiled::new(&crate::schema::DOC);

pub struct Db {
  pub index: Index,
}

pub fn open(path: impl AsRef<Path>) -> Result<Db> {
  Ok(Db {
    index: crate::open(path, &crate::schema::DOC)?,
  })
}

impl Db {
  pub fn searcher(&self) -> Result<Searcher> {
    let index = &self.index;
    Ok(Searcher::new(
      index.reader()?,
      QueryParser::for_index(index, vec![FIELD.title, FIELD.txt, FIELD.tag_li]),
    ))
  }

  pub fn writer(&self) -> Result<Writer> {
    Ok(Writer {
      inner: self.index.writer(
        2 << 24, // 32MB
      )?,
    })
  }
}

pub struct Writer {
  pub inner: IndexWriter<TantivyDocument>,
}

impl Writer {
  pub fn rm(&mut self, id: u64) -> Result<(), TantivyError> {
    let inner = &mut self.inner;
    inner.delete_term(Term::from_field_u64(FIELD.id, id));
    inner.commit()?;
    Ok(())
  }

  pub fn add(&mut self, doc: Doc) -> Result<u64, TantivyError> {
    let inner = &mut self.inner;
    inner.delete_term(Term::from_field_u64(FIELD.id, doc.id));
    let doc_id = inner.add_document(FIELD.dump(doc))?;
    // https://fulmicoton.com/posts/behold-tantivy-part2/
    inner.commit()?;
    Ok(doc_id)
  }
}
