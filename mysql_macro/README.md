[‼️]: ✏️README.mdt

# mysql macro: mysql macro for mysql_async

```rust
use mysql_macro::{conn, exe, q, q01, q1};

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  let mail_id: Option<Option<u64>> = q01!(r#"select mailId("a@b.c")"#);
  dbg!(mail_id);
  let mail_id: Option<u64> = q1!(r#"select mailId("a@b.c")"#);
  dbg!(mail_id);

  exe!(r"select mailHostid('a.com')");

  let mut conn = conn!();

  let mail_host_id: u64 = q1!(&mut conn; r"select mailHostid('a.com')");
  dbg!(mail_host_id);
  let mail_host_id: u64 = q1!(r"select mailHostid('a.com')");
  dbg!(mail_host_id);
  let mail_host_id: u64 = q1!(r"select mailHostid(?)", "a.com");
  dbg!(mail_host_id);
  let mail_host_id: Option<u64> = q01!("select mailHostid('a.com')",);
  dbg!(mail_host_id);

  let q: Vec<Option<u64>> = q!(&mut conn; r"select mailHostid(?)","a.com");
  dbg!(q);

  let q: Vec<(u64,)> = q!(r"select mailHostid(?)", "a.com");
  dbg!(q);

  let q: Vec<u64> = q!(r"select mailHostid(?)", "a.com");
  dbg!(q);

  Ok(())
}
```
