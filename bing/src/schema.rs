use tantivy::schema::{INDEXED, IndexRecordOption, STORED, Schema, TextFieldIndexing, TextOptions};

#[static_init::dynamic]
static TXT: TextOptions = TextOptions::default().set_indexing_options(
  TextFieldIndexing::default()
    .set_tokenizer("zh")
    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
);

pub fn doc() -> Schema {
  let mut build = Schema::builder();

  build.add_u64_field("id", STORED | INDEXED);
  build.add_u64_field("ts", STORED | INDEXED);
  // uid: 用户ID，存储并作为快速字段索引，用于过滤
  build.add_u64_field("uid", STORED | INDEXED);
  // org_id: 组织ID，存储并作为快速字段索引，用于过滤
  build.add_u64_field("org_id", STORED | INDEXED);
  // repo_id: 仓库ID，存储并作为快速字段索引，用于过滤
  build.add_u64_field("repo_id", STORED | INDEXED);
  // tag_li: 标签列表，存储并作为文本索引，用于搜索和过滤
  build.add_text_field("tag_li", STORED | TXT.clone());
  // title: 标题，存储并作为文本索引，用于搜索
  build.add_text_field("title", STORED | TXT.clone());
  // txt: 正文，存储并作为文本索引，用于搜索
  build.add_text_field("txt", STORED | TXT.clone());
  // ts: 时间戳，存储并作为快速字段索引，用于范围查询

  build.build()
}

#[static_init::dynamic]
pub static DOC: Schema = doc();
