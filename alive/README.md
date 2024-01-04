[‼️]: ✏️README.mdt

# alive : use mysql to check is service alive

```rust
/*

#[cfg(feature = "macro")]
mod test_macro {
}
*/

#[tokio::test]
async fn test() -> aok::Result<()> {
  loginit::init();
  alive::next().await?;
  Ok(())
}
```
