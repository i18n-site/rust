use aok::Result;

use crate::{api, api::tran_init_result::State, print_err, Err};

pub async fn print_tran_result(tran_result: api::TranInitResult) -> Result<api::Traning> {
  let state = tran_result.state.unwrap();
  match state {
    State::Traning(traning) => {
      if !traning.update_cache.is_empty() {
        println!(
          "\n✅ update cache\n{}\n──────",
          traning.update_cache.join("\n")
        );
      }

      print_err(&traning.err);
      Ok(traning)
    }
    State::Err(err) => Err(
      Err::Api {
        code: err.code,
        msg: err.msg.clone(),
      }
      .into(),
    ),
  }
}
