use aok::{Result, OK};
use mysql_macro::q;

use crate::db::Status;

pub async fn status() -> Result<()> {
  let li: Vec<Status> =
    q!("SELECT kind_id,host_id,dns_type,err,ts FROM watch ORDER BY err DESC,kind_id,host_id");

  for i in li {
    dbg!(i);
  }
  OK
}
