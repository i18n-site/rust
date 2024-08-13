[‼️]: ✏️README.mdt

# b62

```rust
#[test]
fn test() {
  // let a = b"12345sdfghjuEFEFZ";
  // let b = b62::e(a);
  // let o = b62::d(&b).unwrap();
  // assert_eq!(&a[..], &o[..]);
  for (num, nstr) in [(132222322112u64, "1oqpr8ow"), (0, "0")] {
    let s = b62::num::e36(num);
    println!("{} {}", num, s);
    assert_eq!(s, nstr);
  }
}
```
