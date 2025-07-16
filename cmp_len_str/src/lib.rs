pub fn cmp_len_str<S: AsRef<str>>(a: &S, b: &S) -> std::cmp::Ordering {
  let a = a.as_ref();
  let b = b.as_ref();
  // 先比较字符串长度
  let len_cmp = b.len().cmp(&a.len());
  if len_cmp == std::cmp::Ordering::Equal {
    // 如果长度一致，再按字符串本身的顺序比较
    a.cmp(b)
  } else {
    len_cmp
  }
}
