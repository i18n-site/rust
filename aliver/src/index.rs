// json: Bytes
pub async fn index() -> aerr::msg!() {
  // let subject;
  // let txt;
  // let status;
  // match sonic_rs::from_slice::<mail::Root>(&json) {
  //   Err(e) => {
  //     txt = String::from_utf8_lossy(&json);
  //     let e = e.to_string();
  //     subject = format!("mailhook json parse error : {}", &e);
  //     tracing::error!("{}\n{}", e, &txt);
  //     status = StatusCode::BAD_REQUEST;
  //   }
  //   Ok(root) => {
  //     status = StatusCode::OK;
  //     let payload = root.payload;
  //     subject = payload.subject;
  //     txt = payload.txt.into();
  //   }
  // }
  // if status == StatusCode::OK {
  //   Ok(())
  // } else {
  //   aerr::err(status, ())
  // }

  Ok(sonic_rs::to_string(&alive::status().await?)?)
}
