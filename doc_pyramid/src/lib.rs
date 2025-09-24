use std::future::Future;

use aok::Result;
use doc_chunk::DocChunk;
use futures::{StreamExt, TryStreamExt, stream};

pub async fn digest<AsyncString: Future<Output = Result<String>>>(
  txt: &str,
  doc_chunk: DocChunk,
  short: &impl Fn(String) -> AsyncString,
) -> Result<String> {
  let li = stream::iter(doc_chunk.parse(txt))
    .map(short)
    .buffered(16)
    .try_collect::<Vec<String>>()
    .await?;
  Ok(li.join("\n"))
}

pub async fn parse<AsyncString: Future<Output = Result<String>>>(
  txt: impl AsRef<str>,
  limit: usize,
  short: impl Fn(String) -> AsyncString,
) -> Result<Vec<String>> {
  let mut txt = txt.as_ref();
  let mut result = vec![];
  let doc_chunk = DocChunk::new(limit);
  while !txt.is_empty() {
    let brief = digest(txt, doc_chunk, &short).await?;
    let stop = brief.len() <= limit;
    result.push(brief);
    if stop {
      break;
    } else {
      txt = result.last().unwrap().as_str();
    }
  }

  Ok(result)
}
