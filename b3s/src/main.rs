use std::process::exit;

use aok::{Result, OK};
use base64::prelude::{Engine, BASE64_STANDARD};
use cget::cget;
use clap::{arg, crate_version, Command};
use current_platform::CURRENT_PLATFORM;
use ed25519_dalek::SigningKey;

genv::def!(B3S_SK);

/*
https://docs.rs/ed25519-dalek/latest/ed25519_dalek/
*/
// .arg(
//   arg!(
//             -c --config <FILE> "Sets a custom config file"
//         )
//         // We don't have syntax yet for optional options, so manually calling `required`
//         .required(false)
//         .value_parser(value_parser!(PathBuf)),
// )
// .subcommand(
//   Command::new("test")
//     .about("does testing things")
//     .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
// )

#[tokio::main]
async fn main() -> Result<()> {
  let m = Command::new("b3s")
    .disable_version_flag(true)
    .arg(arg!(-c --create "create key if not exist"))
    .arg(arg!(-k --key <key> "key file path").required(false))
    .arg(arg!(-v - -version))
    .arg(arg!(
        - -vv "more version info"
    ))
    .arg(arg!(<fp> "file path"))
    .get_matches();

  if m.get_one("version") == Some(&true) {
    println!(crate_version!());
    exit(0);
  }

  if m.get_one("vv") == Some(&true) {
    println!(
      r#"ver:{}
build_target:{}"#,
      crate_version!(),
      CURRENT_PLATFORM
    );
    exit(0);
  }

  cget!(
    m:
      fp: String;
      create: bool;
  );

  let key = if let Some(key) = m.get_one::<String>("key") {
    b3s::key(key, *create).await?
  } else {
    let sk: String = B3S_SK();
    let key = &BASE64_STANDARD.decode(sk)?[..];
    SigningKey::from_bytes(&key.try_into()?)
  };

  b3s::b3s(fp, key).await?;
  OK
}
