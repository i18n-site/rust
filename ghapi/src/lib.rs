use std::{collections::HashSet, error::Error, fmt};

use aok::Result;
use reqwest::{Method, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
  pub access_token: String,
  pub token_type: String,
  pub scope: String,
}

#[derive(Debug)]
pub struct GitHubError {
  pub code: StatusCode,
  pub msg: String,
}

impl fmt::Display for GitHubError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "GitHub {} : {}", self.code, self.msg,)
  }
}

impl Error for GitHubError {}

async fn request(token: &str, method: Method, url: &str) -> Result<String> {
  let url = format!("https://api.github.com/{url}");
  let res = ireq::REQ
      .request(method, url)
      .header("Authorization", format!("token {}", token))
      // github 不设置 User-Agent 会报错
      .header("User-Agent", "i")
      .send()
      .await?;

  if res.status().is_success() {
    Ok(res.text().await?)
  } else {
    Err(
      GitHubError {
        code: res.status(),
        msg: res.text().await?,
      }
      .into(),
    )
  }
}

async fn json<T: serde::de::DeserializeOwned>(token: &str, method: Method, url: &str) -> Result<T> {
  let text = request(token, method, url).await?;
  Ok(sonic_rs::from_str(&text)?)
}

#[derive(Debug, Clone)]
pub struct User {
  pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
  pub login: String,
  pub id: u64,
  pub followers: u64,
  pub following: u64,
  pub created_at: String,
  pub updated_at: String,
  pub twitter_username: Option<String>,
  pub name: Option<String>,
  pub company: Option<String>,
  // pub avatar_url: String,
  // pub url: String,
  // pub html_url: String,
  // pub followers_url: String,
  // pub following_url: String,
  // pub gists_url: String,
  // pub starred_url: String,
  // pub subscriptions_url: String,
  // pub organizations_url: String,
  // pub repos_url: String,
  // pub events_url: String,
  // pub received_events_url: String,
  // pub r#type: String,
  // pub site_admin: bool,
  // pub blog: Option<String>,
  // pub location: Option<String>,
  // pub email: Option<String>,
  // pub hireable: Option<bool>,
  // pub bio: Option<String>,
  // pub public_repos: u32,
  // pub public_gists: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserEmail {
  pub email: String,
  pub primary: bool,
}

#[derive(Debug, Deserialize)]
pub struct _UserEmail {
  pub email: String,
  pub primary: bool,
  pub verified: bool,
  pub visibility: Option<String>,
}

impl User {
  pub fn new(token: impl Into<String>) -> User {
    User {
      token: token.into(),
    }
  }

  pub async fn star_user_repo<S1: AsRef<str>, S2: AsRef<str>>(
    &self,
    user_li: impl IntoIterator<Item = S1>,
    repo_li: impl IntoIterator<Item = S2>,
  ) -> Result<()> {
    macro_rules! run {
      ($li:ident, $func:ident) => {
        for i in $li {
          let i = i.as_ref();
          let r = self.$func(i).await;
          if let Err(ref err) = r {
            if let Some(err) = err.downcast_ref::<GitHubError>() {
              if err.code == StatusCode::UNAUTHORIZED {
                return r;
              }
            }
            tracing::error!("{} {} {}", stringify!($func), i, err);
          }
        }
      };
    }

    run!(user_li, follow);
    run!(repo_li, star_repo);
    Ok(())
  }

  pub async fn request(&self, method: Method, url: impl AsRef<str>) -> Result<String> {
    request(&self.token, method, url.as_ref()).await
  }

  pub async fn json<T: serde::de::DeserializeOwned>(&self, url: impl AsRef<str>) -> Result<T> {
    json(&self.token, Method::GET, url.as_ref()).await
  }

  pub async fn info(&self) -> Result<UserInfo> {
    self.json("user").await
  }

  pub async fn _emails(&self) -> Result<Vec<_UserEmail>> {
    self.json("user/emails").await
  }

  pub async fn emails(&self) -> Result<Vec<UserEmail>> {
    Ok(
      self
        ._emails()
        .await?
        .into_iter()
        .filter_map(|e| {
          if e.email.ends_with("@users.noreply.github.com") {
            return None;
          }
          if e.verified {
            Some(UserEmail {
              email: e.email,
              primary: e.primary,
            })
          } else {
            None
          }
        })
        .collect(),
    )
  }

  pub async fn star_repo(&self, owner_repo: impl AsRef<str>) -> Result<()> {
    let url = format!("user/starred/{}", owner_repo.as_ref());
    self.request(Method::PUT, url).await?;
    Ok(())
  }

  pub async fn follow(&self, username: impl AsRef<str>) -> Result<()> {
    let url = format!("user/following/{}", username.as_ref());
    self.request(Method::PUT, &url).await?;
    Ok(())
  }
}

pub fn verify_access_token(access_token: AccessTokenResponse, scope_li: &[&str]) -> Option<String> {
  if access_token.token_type == "bearer" {
    let set: HashSet<&str> = HashSet::from_iter(access_token.scope.split(','));
    for i in scope_li {
      if !set.contains(i) {
        return None;
      }
    }
    return Some(access_token.access_token);
  }
  None
}

pub async fn access_token(
  client_id: impl AsRef<str>,
  client_secret: impl AsRef<str>,
  code: impl AsRef<str>,
) -> Result<AccessTokenResponse> {
  let params = [
    ("client_id", client_id.as_ref()),
    ("client_secret", client_secret.as_ref()),
    ("code", code.as_ref()),
  ];

  let res = ireq::REQ
    .post("https://github.com/login/oauth/access_token")
    .header("Accept", "application/json")
    .form(&params)
    .send()
    .await?
    .text()
    .await?;

  let res = sonic_rs::from_str::<AccessTokenResponse>(&res)?;

  Ok(res)
}
