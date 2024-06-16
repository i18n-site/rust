[‼️]: ✏️README.mdt

# cookiestr

```rust
use anyhow::Result;

#[test]
fn test_replace() -> Result<()> {
  let v = [1, 2, 3, 0];
  let e = cookiestr::e(v);
  println!("{:?} → {}", v, &e);
  let d = cookiestr::d(&e)?;
  assert_eq!(&v, &d[..]);
  Ok(())
}
```
