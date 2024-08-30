pub struct Atom {
  title: String,
  li: Vec<String>,
  host: String,
  ts: String,
}

impl Atom {
  pub fn new(title: impl Into<String>, host: impl Into<String>) -> Self {
    Self {
      ts: Default::default(),
      host: host.into(),
      title: title.into(),
      li: Vec::new(),
    }
  }

  pub fn add(&mut self, ts: u64, url: &str, title: &str, htm: &str) {
    let ts = tsfmt::utc(ts);
    let entry = format!(
      r#"<entry><id>{url}</id><title>{title}</title><link href="https://{host}/{url}"/><updated>{ts}</updated><content type="html"><![CDATA[{htm}]]></content></entry>"#,
      host = self.host,
      url = url,
      title = title,
      ts = ts,
      htm = htm
    );
    if self.ts.is_empty() {
      self.ts = ts;
    }
    self.li.push(entry);
  }

  pub fn gen(&self) -> String {
    let li = self.li.join("");
    format!(
      r#"<?xml version="1.0" encoding="UTF-8"?><feed xmlns="http://www.w3.org/2005/Atom"><title>{title}</title><updated>{ts}</updated>{li}</feed>"#,
      title = self.title,
      ts = self.ts,
    )
  }
}
