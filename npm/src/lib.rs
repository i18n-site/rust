use std::{
  env::{self, temp_dir},
  fmt,
  fs::{remove_file, File},
  io::{BufRead, BufReader},
  path::Path,
};

mod payload;
use aok::{Null, Result, OK};
use defer_lite::defer;
use flate2::{write::GzEncoder, Compression};
use payload::payload;
use reqwest::Client;
use sonic_rs::to_string;
use tar::Builder;

genv::def!(NPM_TOKEN:String|String::default());
genv::s!(NPM_REGISTRY:String|"registry.npmjs.org".into());

#[derive(Debug)]
pub struct Publish {
  pub name: String,
  pub msg: String,
}

#[derive(Debug)]
pub enum Error {
  Publish(Publish),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Publish(c) => write!(f, "{} → {} : {}", &*NPM_REGISTRY, c.name, c.msg),
    }
  }
}

impl std::error::Error for Error {}

pub fn token() -> String {
  let mut token = NPM_TOKEN();
  if token.is_empty() {
    let home = dirs::home_dir();
    if let Some(home) = home {
      let npmrc = home.join(".npmrc");
      if npmrc.exists() {
        if let Ok(file) = xerr::ok!(File::open(npmrc)) {
          let reader = BufReader::new(file);
          for line in reader.lines().map_while(Result::ok) {
            let registry = &*NPM_REGISTRY;
            let prefix = "//".to_owned() + registry + "/:_authToken=";
            if line.starts_with(&prefix) {
              if let Some(p) = line.find('=') {
                token = line[p + 1..].trim().to_owned();
                break;
              }
            }
          }
        }
      }
    }
  }
  token
}

pub fn tgz(src: impl AsRef<Path>, package_json: &Path, out: impl AsRef<Path>) -> Result<()> {
  let out = out.as_ref();
  let src = src.as_ref();
  if out.exists() {
    remove_file(out)?;
  }

  let tar_gz = File::create(out)?;
  let enc = GzEncoder::new(tar_gz, Compression::default());
  let mut tar = Builder::new(enc);

  let package = Path::new("package");
  tar.append_file(package.join("package.json"), &mut File::open(package_json)?)?;

  // Append the directory contents, but adjust the path
  for entry in std::fs::read_dir(src)? {
    let entry = entry?;
    let path = entry.path();
    let name = path.strip_prefix(src).unwrap();
    let package_path = package.join(name);
    if path.is_dir() {
      tar.append_dir_all(&package_path, &path)?;
    } else {
      tar.append_file(&package_path, &mut File::open(path)?)?;
    }
  }

  Ok(())
}

pub fn client() -> reqwest::Result<Client> {
  let mut client_builder = Client::builder();
  if let Ok(proxy) = env::var("https_proxy") {
    client_builder = client_builder.proxy(reqwest::Proxy::https(&proxy)?);
  } else if let Ok(proxy) = env::var("all_proxy") {
    client_builder = client_builder.proxy(reqwest::Proxy::all(&proxy)?);
  }
  let client = client_builder.build()?;
  Ok(client)
}

pub async fn publish(token: &str, src: impl AsRef<Path>, package_json: impl AsRef<Path>) -> Null {
  let package_json = package_json.as_ref();
  let src = src.as_ref();
  let tmp_dir = temp_dir();
  let tfp = tmp_dir.join("0");

  defer! {
    xerr::log!(std::fs::remove_file(&tfp));
  };

  tgz(src, package_json, &tfp)?;
  // let package_json = src.join("package.json");
  let payload = payload(package_json, &tfp)?;
  let name = &payload.name;
  let payload = to_string(&payload)?;

  let url = format!("https://{}/{}", &*NPM_REGISTRY, name.replace('/', "%2f"));

  let response = client()?
    .put(url)
    .body(payload)
    .header("Content-Type", "application/json")
    .bearer_auth(token)
    .send()
    .await?;

  if !response.status().is_success() {
    return Err(
      Error::Publish(Publish {
        name: name.into(),
        msg: response.text().await?,
      })
      .into(),
    );
  }
  OK
}
