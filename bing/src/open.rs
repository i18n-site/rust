// use std::{fs::create_dir_all, path::Path};
//
// use anyhow::Result;
// use tantivy::{
//   Index, IndexSettings,
//   directory::MmapDirectory,
//   schema::Schema,
//   store::{Compressor, ZstdCompressor},
//   tokenizer::{LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer},
// };
//
// pub fn open(path: impl AsRef<Path>, schema: &Schema) -> Result<Index> {
//   let path = path.as_ref();
//
//   if !path.exists() {
//     create_dir_all(path)?;
//   }
//
//   let settings = IndexSettings {
//     docstore_compression: Compressor::Zstd(ZstdCompressor {
//       compression_level: Some(3),
//     }),
//     ..Default::default()
//   };
//   let index = Index::builder()
//     .schema(schema.clone())
//     .settings(settings)
//     .open_or_create(MmapDirectory::open(path)?)?;
//
//   /*
//     RemoveLongFilter 移除超过指定字节数（在 UTF-8 表示中）的标记。
//     在索引无约束内容时特别有用。例如包含 base-64 编码图片的邮件等。
//   */
//
//   let analyzer = TextAnalyzer::builder(tantivy_jieba::JiebaTokenizer {})
//     .filter(RemoveLongFilter::limit(40))
//     .filter(LowerCaser)
//     .filter(Stemmer::default())
//     .build();
//
//   index.tokenizers().register("zh", analyzer);
//
//   Ok(index)
// }
