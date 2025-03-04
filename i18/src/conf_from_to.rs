use std::cmp::Reverse;

use ft::FromTo;
use i18_conf::I18nConf;

pub type RelFt = (String, FromTo);

pub fn conf_from_to(conf: &I18nConf) -> Vec<RelFt> {
  let global = ("".into(), FromTo::from_iter(conf.fromTo.iter()));
  if let Some(path) = &conf.path {
    let mut r = Vec::with_capacity(path.len() + 1);
    for (rel, c) in path {
      r.push((rel.into(), FromTo::from_iter(c.fromTo.iter())));
    }
    r.sort_by_key(|s: &RelFt| Reverse(s.0.len()));
    // 全局前缀要放最后
    r.push(global);
    r
  } else {
    vec![global]
  }
}
