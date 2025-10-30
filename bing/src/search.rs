use anyhow::Result;
use sts;
use tantivy::{
  IndexReader, TantivyDocument, Term,
  collector::{Count, TopDocs},
  query::{BooleanQuery, QueryParser, TermQuery},
  schema::{IndexRecordOption, Value},
  snippet::SnippetGenerator,
};

use crate::doc::{Doc, FIELD};

/// 从 TantivyDocument 中获取 u64 类型的值，如果不存在则返回默认值。
fn u64_from_doc(doc: &TantivyDocument, field: tantivy::schema::Field) -> u64 {
  doc
    .get_first(field)
    .and_then(|v| v.as_u64())
    .unwrap_or_default()
}

/// 从 TantivyDocument 中获取 String 类型的值，如果不存在则返回默认值。
fn str_from_doc(doc: &TantivyDocument, field: tantivy::schema::Field) -> String {
  doc
    .get_first(field)
    .and_then(|v| v.as_str())
    .unwrap_or_default()
    .to_string()
}

/// 从 TantivyDocument 中获取 Vec<String> 类型的值，如果不存在则返回空 Vec。
fn li_from_doc(doc: &TantivyDocument, field: tantivy::schema::Field) -> Vec<String> {
  doc
    .get_all(field)
    .filter_map(|v| v.as_str().map(String::from))
    .collect()
}

pub struct Searcher {
  pub reader: IndexReader,
  pub parser: QueryParser,
  pub ts: u64,
}

pub type Snippet = String;

#[derive(Debug)]
pub struct SearchResult {
  pub li: Vec<(Doc, Snippet)>,
  /// 当 offset = 0 时返回
  pub count: usize,
}

#[derive(Debug)]
pub struct Query {
  pub query: String,
  pub uid: u64,
  pub org_id: u64,
  pub repo_id_li: Vec<u64>,
  pub tag_li: Vec<String>,
  pub ts_begin: Option<u64>,
  pub ts_end: Option<u64>,
  pub limit: usize,
  pub offset: usize,
  pub snippet_max_num_chars: u64,
}

impl Default for Query {
  fn default() -> Self {
    Self {
      query: String::default(),
      uid: u64::default(),
      org_id: u64::default(),
      repo_id_li: Vec::default(),
      tag_li: Vec::default(),
      ts_begin: Option::default(),
      ts_end: Option::default(),
      limit: 10,
      offset: usize::default(),
      snippet_max_num_chars: 300,
    }
  }
}

impl Searcher {
  pub fn new(reader: IndexReader, parser: QueryParser) -> Self {
    Self {
      reader,
      parser,
      ts: sts::sec(),
    }
  }

  pub fn search(&mut self, q: Query) -> Result<SearchResult> {
    let now = sts::sec();
    if now > self.ts {
      self.reader.reload()?;
      self.ts = now;
    }
    let searcher = self.reader.searcher();
    let mut query_li = vec![];

    // 解析原始查询字符串
    let query = q.query;
    if !query.is_empty() {
      query_li.push(self.parser.parse_query(&query)?);
    }

    // uid 和 org_id 过滤
    for (val, field) in [(q.uid, FIELD.uid), (q.org_id, FIELD.org_id)] {
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
      for repo_id_val in q.repo_id_li {
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
    for tag_val in q.tag_li {
      query_li.push(Box::new(TermQuery::new(
        Term::from_field_text(FIELD.tag_li, tag_val.as_ref()),
        IndexRecordOption::Basic,
      )));
    }

    // ts_begin 和 ts_end 过滤
    if let (Some(ts_b), Some(ts_e)) = (q.ts_begin, q.ts_end) {
      query_li.push(self.parser.parse_query(&format!("ts:[{ts_b} TO {ts_e}]"))?);
    } else if let Some(ts_b) = q.ts_begin {
      query_li.push(self.parser.parse_query(&format!("ts:[{ts_b} TO *]"))?);
    } else if let Some(ts_e) = q.ts_end {
      query_li.push(self.parser.parse_query(&format!("ts:[* TO {ts_e}]"))?);
    }

    let boolean_query = BooleanQuery::intersection(query_li);
    let limit = TopDocs::with_limit(q.limit);
    let (top_li, count) = if q.offset == 0 {
      searcher.search(&boolean_query, &(limit, Count))?
    } else {
      (
        searcher.search(&boolean_query, &limit.and_offset(q.offset))?,
        0,
      )
    };

    let mut li = Vec::with_capacity(top_li.len());
    let query_for_snippet = self.parser.parse_query(query.as_ref())?;
    let mut snippet_generator =
      SnippetGenerator::create(&searcher, &*query_for_snippet, FIELD.txt)?;
    snippet_generator.set_max_num_chars(q.snippet_max_num_chars as usize);

    for (_score, doc_address) in &top_li {
      let retrieved_doc = searcher.doc::<TantivyDocument>(*doc_address)?;
      let doc = Doc {
        id: u64_from_doc(&retrieved_doc, FIELD.id),
        ts: u64_from_doc(&retrieved_doc, FIELD.ts),
        uid: u64_from_doc(&retrieved_doc, FIELD.uid),
        org_id: u64_from_doc(&retrieved_doc, FIELD.org_id),
        repo_id: u64_from_doc(&retrieved_doc, FIELD.repo_id),
        tag_li: li_from_doc(&retrieved_doc, FIELD.tag_li),
        title: str_from_doc(&retrieved_doc, FIELD.title),
        txt: str_from_doc(&retrieved_doc, FIELD.txt),
      };
      let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);
      li.push((doc, snippet.to_html()));
    }
    Ok(SearchResult { li, count })
  }
}
