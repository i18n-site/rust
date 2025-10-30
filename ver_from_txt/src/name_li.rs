use tracing::warn;

pub fn name_li(txt: &str) -> Vec<String> {
  let mut li: Vec<String> = Vec::new();

  for part in txt.split(',') {
    if let Some((begin_str, end_str)) = part.split_once('~') {
      let start_res = begin_str.parse::<u64>();
      let end_res = end_str.parse::<u64>();

      // 如果任一解析失败，则不执行任何操作，静默跳过这个无效的范围
      if let (Ok(start), Ok(end)) = (start_res, end_res) {
        for n in start..=end {
          li.push(n.to_string());
        }
      } else {
        warn!("txt invalid range {}", part);
      }
    } else {
      li.push(part.to_string());
    }
  }

  li
}
