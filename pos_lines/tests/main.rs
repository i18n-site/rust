use pos_lines::PosLines;

#[test]
fn test_comment() {
  let text = "<!-- 这是\n多行注释\n-->没有内容<!---->";
  let lines: Vec<_> = PosLines::new(text).collect();

  assert_eq!(
    lines,
    [
      (0, "<!-- 这是",),
      (12, "多行注释",),
      (25, "-->没有内容<!---->",)
    ]
  );
}

#[test]
fn test_empty_text() {
  let text = "";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![]);
}

#[test]
fn test_single_line() {
  let text = "Hello World";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Hello World")]);
}

#[test]
fn test_multiple_lines() {
  let text = "Line 1\nLine 2\nLine 3";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (7, "Line 2"), (14, "Line 3"),]);
}

#[test]
fn test_empty_lines() {
  let text = "Line 1\n\nLine 3";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (8, "Line 3"),]);
}

#[test]
fn test_mixed_line_endings() {
  let text = "Line 1\r\nLine 2\rLine 3\nLine 4";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(
    lines,
    vec![(0, "Line 1"), (8, "Line 2"), (15, "Line 3"), (22, "Line 4"),]
  );
}

#[test]
fn test_consecutive_line_endings() {
  let text = "Line 1\r\r\nLine 2\n\n\nLine 3";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (9, "Line 2"), (18, "Line 3"),]);
}

#[test]
fn test_starting_with_newlines() {
  let text = "\n\r\nLine 1\nLine 2";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(3, "Line 1"), (10, "Line 2"),]);
}

#[test]
fn test_ending_with_newlines() {
  let text = "Line 1\nLine 2\n\r\n";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (7, "Line 2"),]);
}

#[test]
fn test_chinese_characters() {
  let text = "你好\n世界\r\n测试";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "你好"), (7, "世界"), (15, "测试"),]);
}

#[test]
fn test_only_newlines() {
  let text = "\n\r\n\r\n";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![]);
}

#[test]
fn test() {
  let text = "前面的文字\n```\n这是代码块\n包含多行\n```\n后面的文字";
  for (pos, line) in PosLines::new(text) {
    println!("{}: >{}<", pos, line);
  }
}
