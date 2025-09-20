pub fn sub_host(host: impl AsRef<str>) -> Option<String> {
  let host = host.as_ref();
  if let Some(p) = host.find('.') {
    let p = p + 1;
    if p < host.len() {
      let host = &host[p..];
      if host.contains('.') {
        return Some(host.into());
      }
    }
  }
  None
}
