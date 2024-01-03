use std::string::ToString;

use aok::OK;
use sonic_rs::{json, to_string, Value};

genv::s!(LARK_BOT);

pub async fn send(
  title: impl AsRef<str>,
  txt: impl ToString,
  url: impl AsRef<str>,
) -> aok::Result<()> {
  let title = title.as_ref();
  let txt = txt.to_string() + "\n\n";
  let url = url.as_ref();

  let mut li: Vec<Value> = Vec::with_capacity(2);

  li.push(json!({"tag":"text","text":txt}));

  if !url.is_empty() {
    li.push(json!({"tag":"a","text":url,"href":url}));
  }

  let msg = json!({"msg_type":"post","content":{"post":{"zh_cn":{"title":title,"content":[li]}}}});

  let url: &str = LARK_BOT.as_ref();
  let res = ireq::post(url, to_string(&msg)?).await?;
  dbg!(&res);
  OK
}
