pub fn hsec(sec: u64) -> String {
  let minutes = sec / 60;
  if minutes < 99 {
    return format!("{} 分钟", minutes);
  }

  let hours = sec / 3600;
  if hours < 99 {
    return format!("{} 小时", hours);
  }

  format!("{} 天", sec / 86400)
}
