use aok::Result;
use mreq::Mreq;
use prost::Message;

use crate::{
  api::TranResult,
  tran::{API, API_TRAN},
};

pub async fn fetch_tran(id: &str) -> Result<TranResult> {
  let mut req = Mreq::new(&API[..], []);
  let r = req.get(format!("{}/{}", API_TRAN, id)).await?;
  Ok(TranResult::decode(r)?)
}
