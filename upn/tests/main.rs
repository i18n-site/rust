use upn::is_full_width_break;

#[test]
fn test_full_width_break_characters() {
  // 符合 is_full_width_break 的全角标点
  let full_width_break_chars = vec![
    '’', '”', '…', '、', '。', '〉', '》', '』', '】', '﹑', '！', '）', '，', '．', '：', '；',
    '？', '｝',
  ];

  for &c in &full_width_break_chars {
    assert!(
      is_full_width_break(c),
      "字符 '{}' 应该被识别为全角换行标点",
      c
    );
  }
}

#[test]
fn test_non_full_width_break_characters() {
  // 不符合 is_full_width_break 的非全角标点
  let non_full_width_break_chars = vec!['a', '1', '.', ',', '-', '_', ' ', '(', '[', 'A'];

  for &c in &non_full_width_break_chars {
    assert!(
      !is_full_width_break(c),
      "字符 '{}' 不应被识别为全角换行标点",
      c
    );
  }
}
