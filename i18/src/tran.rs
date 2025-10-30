use aok::Result;
use mreq::Mreq;
use prost::Message;
use reqwest::StatusCode;
use static_init::dynamic;

use crate::{api::TranInitResult, Err};

pub static API_TRAN: &str = "tran";

#[dynamic]
pub static API: Vec<String> = {
  if let Ok(host_li) = std::env::var("API") {
    host_li.split(' ').map(|i| i.into()).collect()
  } else {
    [
      "s.i18n.site",
      "s1.018007.xyz",
      "s1.3ti.site",
      "s2.018007.xyz",
      "s2.3ti.site",
      "s3.018007.xyz",
    ]
    .into_iter()
    .map(|i| i.into())
    .collect()
  }
};

pub async fn tran(token: &str, id: &str, body: Vec<u8>) -> Result<TranInitResult> {
  let mut req = Mreq::new(&API[..], [("t", token)]);

  let tran_result = match req.post(format!("{}/{}", API_TRAN, id), body).await {
    Ok(r) => TranInitResult::decode(r)?,
    Err(e) => {
      if let Some(e) = e.downcast_ref::<mreq::Error>() {
        if let mreq::Error::Status { code, .. } = e
          && *code == StatusCode::UNAUTHORIZED
        {
          return Err(Err::Token.into());
        }
      }
      return Err(e);
    }
  };

  Ok(tran_result)
}
