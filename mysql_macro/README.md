[‼️]: ✏️README.mdt

# mysql macro: mysql macro for mysql_async

```rust
use std::collections::HashMap;

use mysql_macro::{conn, e, id_v, q, q01, q1};

#[tokio::test]
async fn main() -> aok::Result<()> {
  loginit::init();
  tracing::debug!("test");
  let id_li = vec![1];
  let li: HashMap<_, String> = id_v("payBrand", id_li).await?;
  dbg!(li);
  let li: HashMap<_, String> = id_v("payBrand", *&[1]).await?;
  dbg!(li);

  // let mail_id: Option<Option<u64>> = q01!(r#"select mailId("a@b.c")"#);
  // dbg!(mail_id);
  // let mail_id: Option<u64> = q1!(r#"select mailId("a@b.c")"#);
  // dbg!(mail_id);
  //
  // e!(r"select mailHostid('a.com')");
  //
  // let mut conn = conn!();
  //
  // let mail_host_id: u64 = q1!(&mut conn; r"select mailHostid('a.com')");
  // dbg!(mail_host_id);
  // let mail_host_id: u64 = q1!(r"select mailHostid('a.com')");
  // dbg!(mail_host_id);
  // let mail_host_id: u64 = q1!(r"select mailHostid(?)", "a.com");
  // dbg!(mail_host_id);
  // let mail_host_id: Option<u64> = q01!("select mailHostid('a.com')",);
  // dbg!(mail_host_id);
  //
  // let q: Vec<Option<u64>> = q!(&mut conn; r"select mailHostid(?)","a.com");
  // dbg!(q);
  //
  // let q: Vec<(u64,)> = q!(r"select mailHostid(?)", "a.com");
  // dbg!(q);
  //
  // let q: Vec<u64> = q!(r"select mailHostid(?)", "a.com");
  // dbg!(q);

  // let s = r#"'\'test\''"#;
  // println!("{}", mysql_macro::s(s));
  //
  // let s = [211, 222, 223, 224, 225];
  // println!("{}", mysql_macro::b(&s[..]));
  Ok(())
}
```
