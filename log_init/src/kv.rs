use std::borrow::Cow;

use logforth::diagnostic::Visitor;

pub struct Kv {
  pub text: String,
}

impl<'kvs> log::kv::VisitSource<'kvs> for Kv {
  fn visit_pair(
    &mut self,
    key: log::kv::Key<'kvs>,
    value: log::kv::Value<'kvs>,
  ) -> Result<(), log::kv::Error> {
    use std::fmt::Write;

    write!(&mut self.text, " {key}={value}")?;
    Ok(())
  }
}

impl Visitor for Kv {
  fn visit(&mut self, key: Cow<str>, value: Cow<str>) -> anyhow::Result<()> {
    use std::fmt::Write;

    write!(&mut self.text, " {key}={value}")?;
    Ok(())
  }
}
