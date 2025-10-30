use std::{borrow::Borrow, fmt, ops::Deref, time::Duration};

use reqwest::redirect;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::error::Result;

pub struct Proxy {
  pub name: String,
  pub client: ClientWithMiddleware,
}

impl Proxy {
  pub fn new(name: &str, url: &str) -> Result<Self> {
    let client = reqwest::Client::builder()
      .no_proxy()
      .redirect(redirect::Policy::limited(10))
      .timeout(Duration::from_secs(*reqwest_client::TIMEOUT))
      .build()?;
    let client: ClientWithMiddleware = ClientBuilder::new(client)
      .with(reqwest_proxy::Proxy::from_url(url)?)
      .build();
    Ok(Self {
      name: name.into(),
      client,
    })
  }
}

impl Borrow<String> for Proxy {
  fn borrow(&self) -> &String {
    &self.name
  }
}

impl Deref for Proxy {
  type Target = ClientWithMiddleware;

  fn deref(&self) -> &Self::Target {
    &self.client
  }
}

impl fmt::Display for Proxy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl fmt::Debug for Proxy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Proxy")
      .field("name", &self.name)
      .finish_non_exhaustive()
  }
}
