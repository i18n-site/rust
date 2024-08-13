[‼️]: ✏️README.mdt

# str0

```rust
use roaring::RoaringTreemap;
use static_init::constructor;
use str0::split;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() {
  let li = ["Hello", "skip1", "skip2", "World", "skip3", "!!"];
  let skip_li = RoaringTreemap::from_iter([1, 2, 4]);

  let merged = str0::merge(li, &skip_li);
  assert_eq!(
    merged,
    vec![b'H', b'e', b'l', b'l', b'o', 0, 0, b'W', b'o', b'r', b'l', b'd', 0, b'!', b'!']
  );
  let split_result = split(merged);

  assert_eq!(split_result, ["Hello", "", "", "World", "", "!!"]);
}
```
