use std::path::Path;

use anyhow::Result;
use tantivy::{
  collector::{Count, TopDocs},
  query::{BooleanQuery, QueryParser, TermQuery},
  schema::{Field, IndexRecordOption, Schema, Value},
  Index, IndexReader, IndexWriter, TantivyDocument, TantivyError, Term,
};

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
      tdoc.add_text(self.tag_li, i);
    }
    tdoc.add_u64(self.ts, doc.ts);
    tdoc.add_text(self.title, doc.title);
    tdoc.add_text(self.txt, doc.txt);
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

// pub fn query(&self, query: impl AsRef<str>) -> Result<QueryParser> {
//   let query_parser = QueryParser::for_index(&index, vec![FIELD.title, FIELD.txt, FIELD.tag_li]);
// }

pub struct Searcher {
  pub reader: IndexReader,
  pub parser: QueryParser,
  pub ts: u64,
}

#[derive(Debug)]
pub struct SearchResult {
  pub li: Vec<u64>,
  /// 当 offset = 0 时返回
  pub count: usize,
}

impl Searcher {
  pub fn new(reader: IndexReader, parser: QueryParser) -> Self {
    Self {
      reader,
      parser,
      ts: sts::sec(),
    }
  }

  pub fn search(
    &mut self,
    query: impl AsRef<str>,
    uid: u64,
    org_id: u64,
    repo_id_li: impl IntoIterator<Item = u64>,
    tag_li: impl IntoIterator<Item = String>,
    ts_begin: Option<u64>,
    ts_end: Option<u64>,
    limit: usize,
    offset: usize,
  ) -> Result<SearchResult> {
    let now = sts::sec();
    if now > self.ts {
      self.reader.reload()?;
      self.ts = now;
    }
    let searcher = self.reader.searcher();
    let mut query_li = vec![];

    // 解析原始查询字符串
    let query = query.as_ref();
    if !query.is_empty() {
      query_li.push(self.parser.parse_query(query)?);
    }

    // uid 和 org_id 过滤
    for (val, field) in [(uid, FIELD.uid), (org_id, FIELD.org_id)] {
      if val > 0 {
        query_li.push(Box::new(TermQuery::new(
          Term::from_field_u64(field, val),
          IndexRecordOption::Basic,
        )));
      }
    }

    // repo_id_li 过滤 (OR 逻辑)
    {
      let mut repo_id_filter = vec![];
      for repo_id_val in repo_id_li {
        repo_id_filter.push(Box::new(TermQuery::new(
          Term::from_field_u64(FIELD.repo_id, repo_id_val),
          IndexRecordOption::Basic,
        )) as Box<dyn tantivy::query::Query>);
      }
      if !repo_id_filter.is_empty() {
        query_li.push(Box::new(BooleanQuery::union(repo_id_filter)));
      }
    }

    // tag_li 过滤
    for tag_val in tag_li {
      query_li.push(Box::new(TermQuery::new(
        Term::from_field_text(FIELD.tag_li, tag_val.as_ref()),
        IndexRecordOption::Basic,
      )));
    }

    // ts_begin 和 ts_end 过滤
    if let (Some(ts_b), Some(ts_e)) = (ts_begin, ts_end) {
      query_li.push(
        self
          .parser
          .parse_query(&format!("ts:[{} TO {}]", ts_b, ts_e))?,
      );
    } else if let Some(ts_b) = ts_begin {
      query_li.push(self.parser.parse_query(&format!("ts:[{} TO *]", ts_b))?);
    } else if let Some(ts_e) = ts_end {
      query_li.push(self.parser.parse_query(&format!("ts:[* TO {}]", ts_e))?);
    }

    let query = BooleanQuery::intersection(query_li);
    let (top_li, count) = if offset == 0 {
      searcher.search(&query, &(TopDocs::with_limit(limit), Count))?
    } else {
      (
        searcher.search(&query, &TopDocs::with_limit(limit).and_offset(offset))?,
        0,
      )
    };

    let mut li = Vec::with_capacity(top_li.len());
    for (_score, doc_address) in &top_li {
      let doc = searcher.doc::<TantivyDocument>(*doc_address)?;
      if let Some(id_val) = doc.get_first(FIELD.id)
        && let Some(id) = id_val.as_u64()
      {
        li.push(id);
      }
    }
    Ok(SearchResult { li, count })
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
