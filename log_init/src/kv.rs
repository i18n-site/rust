use std::fmt::Write;

use logforth::kv::{Visitor, value_bag::ValueBag};

pub struct Kv {
  pub text: String,
}

impl<'kvs> log::kv::VisitSource<'kvs> for Kv {
  fn visit_pair(
    &mut self,
    key: log::kv::Key<'kvs>,
    value: log::kv::Value<'kvs>,
  ) -> Result<(), log::kv::Error> {
    write!(&mut self.text, " {key}={value}").unwrap();
    Ok(())
  }
}

impl Visitor for Kv {
  fn visit(
    &mut self,
    key: logforth::kv::Key<'_>,
    value: ValueBag<'_>,
  ) -> Result<(), logforth::Error> {
    write!(&mut self.text, " {key}={value}").unwrap();
    Ok(())
  }
}
