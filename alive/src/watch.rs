use aok::Result;

pub async fn watch(result: Result<()>) {
  if let Err(err) = result {
    dbg!(&err);
  }
}
