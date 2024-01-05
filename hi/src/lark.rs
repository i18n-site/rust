use aok::OK;
use sonic_rs::{json, to_string, Value};
use xstr::cut;

genv::s!(LARK_BOT);

pub async fn send(
  title: impl AsRef<str>,
  txt: impl AsRef<str>,
  url: impl AsRef<str>,
) -> aok::Result<()> {
  let title = cut(title.as_ref(), 255);
  let txt = cut(txt.as_ref(), 10000);
  let url = url.as_ref();

  let mut li: Vec<Value> = Vec::with_capacity(2);

  let txt = txt.to_owned() + "\n";

  li.push(json!({"tag":"text","text":txt}));

  li.push(json!({
    "tag": "at",
    "user_id": "all", //取值使用"all"来at所有人
  }));

  if !url.is_empty() {
    li.push(json!({"tag":"a","text":url,"href":url}));
  };

  let msg = json!({"msg_type":"post","content":{"post":{"zh_cn":{"title":title,"content":[li]}}}});

  let url: &str = LARK_BOT.as_ref();
  ireq::post(url, to_string(&msg)?).await?;
  OK
}
