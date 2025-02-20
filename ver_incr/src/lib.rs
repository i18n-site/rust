#![feature(let_chains)]

pub fn ver_incr(ver: &str) -> String {
  let mut iter = ver.char_indices().rev();
  let mut end = None;
  let mut start = None;

  // 第一步：找到数字的最后一位
  for (i, c) in &mut iter {
    if c.is_ascii_digit() {
      end = Some(i + 1);
      start = Some(i);
      break;
    }
  }

  // 第二步：找到数字的第一位
  if end.is_some() {
    for (i, c) in iter {
      if c.is_ascii_digit() {
        start = Some(i);
      } else {
        break;
      }
    }
  }

  if let Some(start) = start
    && let Some(end) = end
  {
    let num: u64 = ver[start..end].parse().unwrap();
    let new_num = num + 1;
    let new_ver = format!("{}{}{}", &ver[..start], new_num, &ver[end..]);
    new_ver
  } else {
    // 如果没有找到数字部分，在结尾添加 ".0"
    format!("{}.0", ver)
  }
}
