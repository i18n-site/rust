[‼️]: ✏️README.mdt

# xmail

convert mail address to low trim case , use punycode decode domain

```rust
#[test]
fn test() {
  assert_eq!(xmail::norm("  3Ti@Site.xn--yfro4i67o "), "3ti@site.新加坡");
  assert_eq!(
    xmail::norm_tld("  3Ti@Site.aA.xn--yfro4i67o ").1,
    "aa.新加坡"
  )
}
```
