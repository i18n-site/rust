use std::collections::HashMap;

use crate::{api, api::ErrLi};

pub fn print_err(err: &HashMap<i32, ErrLi>) {
  for (err, path_li) in err {
    let err = (*err)
      .try_into()
      .map(|err: api::TranErr| format!("{:?}", err))
      .unwrap_or_else(|_| format!("{}", err));

    let li = &path_li.li;

    eprintln!("\n❌ {err}",);
    crate::print_li::stderr(
      li.len(),
      li.iter().map(|i| {
        if i.msg.is_empty() {
          format!("  {}", i.path)
        } else {
          format!("  {}\n    {}", i.path, i.msg)
        }
      }),
    );
  }
}
