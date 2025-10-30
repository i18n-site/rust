pub fn is_full_width_break(c: char) -> bool {
  matches!(c, |'’'| '”'
    | '…'
    | '、'
    | '。'
    | '〉'
    | '》'
    | '』'
    | '】'
    | '﹑'
    | '！'
    | '）'
    | '，'
    | '．'
    | '：'
    | '；'
    | '？'
    | '｝')
}
