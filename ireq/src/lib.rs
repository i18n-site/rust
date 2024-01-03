use reqwest::Client;

#[static_init::dynamic]
pub static REQ: Client = Client::builder()
  .timeout(Duration::from_secs(60))
  .http3_prior_knowledge()
  .build()
  .unwrap();

pub async fn ireq() {}
