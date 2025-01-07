mod payload;

use std::{
  env::{self, temp_dir},
  fmt, fs,
  fs::{remove_file, File},
  io::{BufRead, BufReader},
  path::Path,
  time::Duration,
};

use aok::Result;
use flate2::{write::GzEncoder, Compression};
use payload::payload;
use reqwest::Client;
use sonic_rs::{to_string, Value};

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

pub fn pkg_with_name(fp: &Path, name: &str) -> Result<String> {
  let mut json_data: Value = sonic_rs::from_str(&fs::read_to_string(fp)?)?;
  json_data["name"] = name.into();
  Ok(sonic_rs::to_string(&json_data)?)
}

pub fn tgz(
  src: impl AsRef<Path>,
  package_json: &Path,
  pkg_name: &str,
  out: impl AsRef<Path>,
) -> Result<()> {
  use tar::{Builder, Header};

  let out = out.as_ref();
  let src = src.as_ref();
  if out.exists() {
    remove_file(out)?;
  }

  let tar_gz = File::create(out)?;
  let enc = GzEncoder::new(tar_gz, Compression::default());
  let mut tar = Builder::new(enc);

  let package = Path::new("package");

  let package_json = pkg_with_name(package_json, pkg_name)?;
  let mut header = Header::new_gnu();
  header.set_size(package_json.len() as u64);
  header.set_cksum();
  tar.append_data(
    &mut header,
    package.join("package.json"),
    package_json.as_bytes(),
  )?;

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
  let mut build = Client::builder();

  build = build
    .timeout(Duration::from_secs(100))
    .connect_timeout(Duration::from_secs(10));
  if let Ok(proxy) = env::var("https_proxy") {
    build = build.proxy(reqwest::Proxy::https(&proxy)?);
  } else if let Ok(proxy) = env::var("all_proxy") {
    build = build.proxy(reqwest::Proxy::all(&proxy)?);
  }
  let client = build.build()?;
  Ok(client)
}

pub enum State {
  Ok,
  VerLow,
}

pub async fn publish(
  token: &str,
  src: impl AsRef<Path>,
  package_json: impl AsRef<Path>,
  pkg_name: &str,
) -> Result<State> {
  let package_json = package_json.as_ref();
  let src = src.as_ref();
  let tmp_dir = temp_dir();
  let tfp = tmp_dir.join("0");

  tgz(src, package_json, pkg_name, &tfp)?;
  // let package_json = src.join("package.json");
  let payload = payload(pkg_name, package_json, &tfp)?;

  std::fs::remove_file(&tfp)?;

  let name = &payload.name;
  let payload = to_string(&payload)?;

  let client = client()?;
  let url = format!("https://{}/{}", &*NPM_REGISTRY, name.replace('/', "%2f"));

  let mut retry = 9;

  while retry > 0 {
    retry -= 1;
    if let Ok(response) = xerr::ok!(
      client
        .put(&url)
        .body(payload.clone())
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send()
        .await
    ) {
      let status = response.status();
      if status.is_success() {
        break;
      } else if retry == 0 {
        return Err(
          Error::Publish(Publish {
            name: name.into(),
            msg: response.text().await.unwrap_or_default(),
          })
          .into(),
        );
      } else if let Ok(text) = xerr::ok!(response.text().await) {
        eprintln!("{} {} → {} : {}", &*NPM_REGISTRY, status, name, text);
        if status == reqwest::StatusCode::FORBIDDEN && text.contains(" published ") {
          return Ok(State::VerLow);
        }
      }
    }
  }
  Ok(State::Ok)
}
