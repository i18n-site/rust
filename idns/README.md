[‼️]: ✏️README.mdt

# idns

```rust
use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  use idns::ping::use_ipv6;

  let use_ipv6 = use_ipv6().await;
  dbg!(use_ipv6);
  // for host in [
  //   "mail.i18n.site",
  //   "baidu.com",
  //   "youdao.com",
  //   "z.com",
  //   "google.com",
  // ] {
  //   let r = idns::A(host).await?;
  //   dbg!((host, r));
  // }
  OK
}
```

out

```

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
