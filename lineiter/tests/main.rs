use lineiter::LineIter;

#[test]
fn test_line_iter() {
  let data = b"line1\nline2\r\nline3\rline4";
  let expected_lines = [
    "line1".to_string(),
    "line2".to_string(),
    "line3".to_string(),
    "line4".to_string(),
  ];

  // let data = b"line1";
  // let expected_lines = ["line1".to_string()];

  let lines = LineIter::new(&data[..]);

  let mut n = 0;

  for (line, expected) in lines.zip(expected_lines.iter()) {
    match line {
      Ok(line) => {
        n += 1;
        dbg!(&line);
        assert_eq!(line, *expected)
      }
      Err(e) => panic!("Error: {}", e),
    }
  }
  assert_eq!(n, expected_lines.len());
}
