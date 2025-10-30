use aiapi::{AiApi, Conf, Gemini};
use aok::{OK, Void};
use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
  #[error("{prompt}\n{content}")]
  VerifyFailed { prompt: String, content: String },
}

pub async fn verify(proxy: &reqwest::Client, conf: &Conf, token: impl AsRef<str>) -> Void {
  let mut rng = rand::rng();
  let n1: u8 = rng.random();
  let n2: u8 = rng.random();
  let prompt = format!("{}+{}=", n1, n2);
  let req = Gemini.req(proxy, conf, "gemma-3-27b-it", &prompt)?;
  let expect = (n1 as u32) + (n2 as u32);
  let result = Gemini.chat(token.as_ref(), &req).await?;

  if !result.content.contains(&expect.to_string()) {
    return Err(
      Error::VerifyFailed {
        prompt,
        content: result.content,
      }
      .into(),
    );
  }

  OK
}
