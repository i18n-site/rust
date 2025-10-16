pub fn hsec(sec: u64) -> String {
  let minutes = sec / 60;
  if minutes < 99 {
    return format!("{minutes} 分钟");
  }

  let hours = sec / 3600;
  if hours < 99 {
    return format!("{hours} 小时");
  }

  format!("{} 天", sec / 86400)
}
