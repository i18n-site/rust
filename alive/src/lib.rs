use aok::{Result, OK};
use mysql_macro::mysql_async::prelude::FromRow;
use xhash::HashSet;
mod m;

#[derive(Debug, Clone, FromRow)]
pub struct Kind {
  pub id: u64,
  pub host_id: u64,
  pub duration: u32,
  #[allow(non_snake_case)]
  pub warnErr: u8,
  pub v: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Watch {
  pub id: u64,
  pub host_id: u64,
  pub kind_id: u64,
  pub dns_type: u8,
  pub err: u32,
  pub url_id: u64,
}

pub async fn next() -> Result<()> {
  let now = sts::sec();

  let li: Vec<Watch> = m::q!(
    "SELECT id,host_id,kind_id,dns_type,err,url_id FROM watch WHERE ts<=?",
    now
  );
  if li.is_empty() {
    return OK;
  }
  let mut kind_set = HashSet::default();
  let mut host_set = HashSet::default();
  let mut url_set = HashSet::default();

  li.iter().for_each(|w| {
    kind_set.insert(w.kind_id);
    host_set.insert(w.host_id);
    if w.url_id != 0 {
      url_set.insert(w.url_id);
    }
  });
  dbg!(li);

  let sql = format!(
    "SELECT id,host_id,duration,warnErr,v FROM kind WHERE id IN ({})",
    kind_set
      .clone()
      .into_iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join(",")
  );

  dbg!(&sql);
  let kind_li: Vec<Kind> = m::q!(format!(
    "SELECT id,host_id,duration,warnErr,v FROM kind WHERE id IN ({})",
    kind_set
      .into_iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join(",")
  ));

  dbg!(kind_li);
  OK
}
