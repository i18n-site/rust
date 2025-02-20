use std::collections::HashMap;

use aok::Result;
use dashmap::DashMap;
use ifs::conf;
use pos_next::PosNext;
use sonic_rs::Deserialize;

use crate::{
  api,
  api::{post, AccessToken, IdAccessToken, MsgLi},
  response::{FinishReason, Usage},
  Api, Response,
};

conf!(BaiduConf {
  id_token: IdAccessToken
});

#[derive(Deserialize, Debug)]
struct ChatCompletion {
  // id: String,
  // object: String,
  // created: u64,
  result: String,
  is_truncated: bool,
  need_clear_history: bool,
  usage: Usage,
}

impl From<ChatCompletion> for Response {
  fn from(v: ChatCompletion) -> Self {
    Self {
      usage: v.usage,
      txt: v.result,
      finish_reason: if v.is_truncated {
        FinishReason::Length
      } else if v.need_clear_history {
        FinishReason::ContentFilter
      } else {
        FinishReason::Stop
      },
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct Conf {
  pub api: String,
  pub temperature: String,
  pub max_tokens: usize,
  pub 上下文: usize,
  pub app: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Baidu {
  pub conf: Conf,
  pub _pos: PosNext,
  pub token_li: DashMap<usize, (String, AccessToken)>,
  pub cache_id_token: conf::Item<IdAccessToken>,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
  // refresh_token: String,
  // scope: String,
  // session_key: String,
  access_token: String,
  expires_in: u64,
}

async fn access_token(client_id: &str, client_secret: &str) -> Result<TokenResponse> {
  let url = format!(
        "https://aip.baidubce.com/oauth/2.0/token?client_id={}&client_secret={}&grant_type=client_credentials",
        client_id, client_secret
    );

  let response = ireq::REQ
    .post(&url)
    // .headers(headers)
    .body("")
    .send()
    .await?
    .text()
    .await?;

  Ok(sonic_rs::from_str(&response)?)
}

// impl Api for Baidu {}
impl Baidu {
  pub fn loads(yml: impl AsRef<[u8]>) -> Result<Self, serde_yaml::Error> {
    let conf: Conf = serde_yaml::from_slice(yml.as_ref())?;
    let cache: BaiduConf = ifs::confdir().join("aiapi").join("baidu").into();
    let cache_id_token = cache.id_token();
    let id_token = cache_id_token.get().unwrap_or_default().id_token;
    Ok(Self {
      _pos: PosNext::new(),
      token_li: conf
        .app
        .iter()
        .enumerate()
        .map(|(pos, (id, _secret))| {
          let token = id_token.get(id).cloned();
          (pos, (id.clone(), token.unwrap_or_default()))
        })
        .collect(),
      conf,
      cache_id_token,
    })
  }
}

impl Api for Baidu {
  async fn send(
    &self,
    system: impl Into<String> + Send,
    msg_li: impl Into<MsgLi> + Send,
  ) -> Result<Response> {
    let msg: String = msg_li.into().into();
    let pos = self._pos.next() % self.conf.app.len();
    let now = sts::sec();
    let system = system.into();
    let system = if !system.is_empty() {
      format!(r#""system":{},"#, sonic_rs::to_string(&system).unwrap())
    } else {
      "".into()
    };
    loop {
      let mut token = self.token_li.get_mut(&pos).unwrap();
      if token.1.expire < (60 + now) {
        let id = &token.0;
        let secret = self.conf.app.get(id).unwrap();
        let t = access_token(id, secret).await?;
        token.1.expire = now + t.expires_in;
        token.1.access_token = t.access_token;
        drop(token);
        let id_token: HashMap<_, _> = self
          .token_li
          .iter()
          .map(|entry| entry.value().clone())
          .collect();
        self.cache_id_token.set(IdAccessToken { id_token });
        continue;
      }

      let access_token = &token.1.access_token;
      let url = format!("{}?access_token={}", self.conf.api, access_token);
      let body = format!(
        r#"{{{system}"max_output_tokens":{},"temperature":{},"messages":[{msg}]}}"#,
        self.max_tokens(),
        self.conf.temperature,
      );
      let r: ChatCompletion = post(url, body, |req| req).await?;
      return Ok(r.into());
    }
  }

  fn max_tokens(&self) -> usize {
    self.conf.max_tokens
  }

  fn 上下文(&self) -> usize {
    self.conf.上下文
  }
}
