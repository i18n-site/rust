use std::{
  collections::{HashMap, HashSet},
  path::Path,
  sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
  },
  time::Duration,
};

use aok::{OK, Result, Void, anyhow};
use seekstorm::{
  commit::Commit,
  index::{
    AccessType, FieldType, FileType, FrequentwordType, IndexArc, IndexDocument, IndexMetaObject,
    SchemaField, SimilarityType, StemmerType, StopwordType, TokenizerType, create_index,
    open_index,
  },
  search::{FacetFilter, QueryType, ResultType, Search},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Doc {
  id: u64,
  org_id: u64,
  repo_id: u64,
  tag_li: Vec<String>,
  title: String,
  ts: u64,
  txt: String,
  uid: u64,
}

impl From<Doc> for HashMap<String, Value> {
  fn from(article: Doc) -> Self {
    let mut map = HashMap::new();
    map.insert("id".into(), Value::from(article.id));
    map.insert("uid".into(), Value::from(article.uid));
    map.insert("org_id".into(), Value::from(article.org_id));
    map.insert("repo_id".into(), Value::from(article.repo_id));
    map.insert("title".into(), Value::from(article.title));
    map.insert("txt".into(), Value::from(article.txt));
    map.insert("ts".into(), Value::from(article.ts));
    map.insert("tag_li".into(), Value::from(article.tag_li));
    map
  }
}

pub fn schema_u64(name: impl Into<String>) -> SchemaField {
  SchemaField::new(
    name.into(),
    false, // stored: 是否在文档中存储字段的原始值
    false, // indexed: 是否索引该字段
    FieldType::U64,
    true, // facet: 是否将该字段作为分面字段
    1.0,
  )
}

pub fn init_db(path: impl AsRef<Path>) -> Void {
  use FieldType::{Text, U64};

  let path: &Path = path.as_ref();

  if path.exists() {
    return OK;
  }
  let schema_li = vec![
    SchemaField::new("id".into(), true, false, U64, true, 1.0),
    SchemaField::new("title".into(), false, true, Text, false, 4.0),
    SchemaField::new("tag_li".into(), false, true, Text, true, 2.0),
    SchemaField::new("txt".into(), false, true, Text, false, 1.0),
    schema_u64("ts"),
    schema_u64("uid"),
    schema_u64("org_id"),
    schema_u64("repo_id"),
  ];

  let meta = IndexMetaObject {
    id: 0,
    name: "rag".into(),
    similarity: SimilarityType::Bm25fProximity,
    tokenizer: TokenizerType::UnicodeAlphanumericZH, // 分词器：支持中文
    stemmer: StemmerType::None,
    stop_words: StopwordType::None,
    frequent_words: FrequentwordType::English,
    access_type: AccessType::Mmap,
  };

  create_index(
    path,
    meta,
    &schema_li,
    true,        // 序列化模式
    &Vec::new(), // 同义词向量
    11,          // segment_number_bits = 11 将这个索引分成 2048 个小块来管理
    false,
  )
  .map_err(|e| anyhow!(e))?;

  OK
}

pub struct Indexer {
  pub index: IndexArc,
  pub need_commit: Arc<AtomicBool>,
  ing: tokio::task::JoinHandle<()>,
}

impl Drop for Indexer {
  fn drop(&mut self) {
    self.ing.abort();
  }
}

impl Indexer {
  pub async fn open(path: impl AsRef<Path>) -> Result<Self> {
    let path: &Path = path.as_ref();
    if !path.exists() {
      init_db(path)?;
    }

    let need_commit = Arc::new(AtomicBool::new(false));
    let index = open_index(path, false).await.map_err(|e| anyhow!(e))?;

    Ok(Self {
      ing: {
        let need_commit = need_commit.clone();
        let index = index.clone();
        tokio::spawn(async move {
          loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if need_commit.swap(false, Ordering::Relaxed) {
              index.commit().await;
            }
          }
        })
      },
      index,
      need_commit,
    })
  }

  pub async fn upsert(&self, article: Doc) {
    self
      .index
      .index_document(article.into(), FileType::None)
      .await;
    self.need_commit.store(true, Ordering::Relaxed);
    // self.index.commit().await;
  }

  // pub async fn rm(&self, id: u64) -> Result<()> {
  //   let mut index = self.index.write().await;
  //   index.remove_document(id).await?;
  //   Ok(())
  // }
}

pub fn u64_eq(name: impl Into<String>, id: u64) -> FacetFilter {
  FacetFilter::U64 {
    field: name.into(),
    filter: id..id + 1,
  }
}

pub async fn search(
  index: &IndexArc,
  query: impl Into<String>,
  offset: usize,
  limit: usize,
  facet_filter: Vec<FacetFilter>,
) -> Result<(Vec<(u64, f32)>, usize)> {
  let query = query.into();
  /*
  https://docs.rs/seekstorm/0.12.27/seekstorm/index/type.IndexArc.html#method.search

  query_string: String
  用途: 这是用户的核心搜索查询字符串。
  说明: 函数能识别并解析查询字符串中的特殊操作符，例如：
  + (AND): 要求必须包含某个词。
  - (NOT): 要求必须排除某个词。
  "..." (PHRASE): 要求短语必须完整匹配。
  */

  let r = index
    .search(
      query,
      QueryType::Union,
      offset,
      limit,
      ResultType::TopkCount,
      true,
      vec![],
      vec![],
      facet_filter,
      vec![],
    )
    .await;

  let mut li = Vec::with_capacity(r.result_count);
  let fields = HashSet::new();
  {
    let index = index.read().await;
    for i in r.results {
      if let Ok(doc) = xerr::ok!(index.get_document(i.doc_id, true, &None, &fields, &[])) {
        if let Some(id) = doc.get("id") {
          li.push((id.as_u64().unwrap(), i.score));
        }
      }
    }
  }
  Ok((li, r.result_count_total))
}

#[tokio::main]
async fn main() -> Void {
  let path = "/tmp/search/rag";

  let indexer = Indexer::open(path).await?;

  // let article = Doc {
  //   id: 1122,
  //   uid: 102,
  //   org_id: 202,
  //   repo_id: 303,
  //   title: "一个关于Rust搜索库的标题".into(),
  //   txt: "这是对Seek Storm搜索库的一个测试。".into(),
  //   ts: 1678886400,
  //   tag_li: vec!["rust".into(), "search".into(), "人类".into()],
  // };
  //
  // indexer.upsert(article).await;

  dbg!(indexer.index.read().await.indexed_doc_count);
  // tokio::time::sleep(Duration::from_secs(1)).await;
  // dbg!(indexer.index.read().await.indexed_doc_count);

  // u64_eq("id", 1122)
  let r = search(&indexer.index, "seek", 0, 2, vec![]).await?;
  dbg!(r);

  OK
}
