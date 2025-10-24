use std::path::Path;

use aok::Result;

pub async fn latest(pkg: impl AsRef<str>, dir: impl AsRef<Path>) -> Result<String> {
  let pkg = pkg.as_ref();
  let dir = dir.as_ref();
  let cache_file = dir.join(pkg);
  let now = sts::sec();

  // 检查缓存文件是否存在
  if cache_file.exists() {
    let s = ifs::rstr(&cache_file)?;
    let mut s = s.split_whitespace();
    if let Some(ver) = s.next()
      && let Some(ts) = s.next()
      && let Ok(ts) = ts.parse::<u64>()
      && now - ts < 24 * 60 * 60
    {
      return Ok(ver.into());
    }
  }

  // 缓存失效或不存在时，使用 npmv::latest 获取版本号
  let version = super::latest(pkg).await?; // 获取最新版本

  ifs::wstr(cache_file, format!("{version} {now}"))?;

  Ok(version)
}
