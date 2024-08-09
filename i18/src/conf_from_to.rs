use std::cmp::Reverse;

use ft::FromTo;
use i18_conf::I18nConf;

pub type RelFt = (String, FromTo);

pub fn conf_from_to(conf: &I18nConf) -> Vec<RelFt> {
  let mut r = Vec::with_capacity(1 + conf.path.len());
  for (rel, c) in &conf.path {
    r.push((rel.into(), FromTo::from_iter(c.fromTo.iter())));
  }
  r.sort_by_key(|s: &RelFt| Reverse(s.0.len()));
  r.push(("".into(), FromTo::from_iter(conf.fromTo.iter())));
  r
}
