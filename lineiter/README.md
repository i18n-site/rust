[‼️]: ✏️README.mdt

# lineiter

```rust
use lineiter::LineIter;

#[test]
fn test_line_iter() {
  let data = b"line1\nline2\r\nline3\rline4";
  let reader = &data[..];
  let lines = LineIter::new(reader);

  let expected_lines = [
    "line1".to_string(),
    "line2".to_string(),
    "line3".to_string(),
    "line4".to_string(),
  ];

  for (line, expected) in lines.zip(expected_lines.iter()) {
    match line {
      Ok(line) => assert_eq!(line, *expected),
      Err(e) => panic!("Error: {}", e),
    }
  }
}
```
