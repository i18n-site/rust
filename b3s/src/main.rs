use aok::{Result, OK};
use base64::prelude::{Engine, BASE64_STANDARD};
use cget::cget;
use clap::arg;
use cmdv::cmdv;
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
  let mut cmd = cmdv("b3s")
    .arg(arg!(-k --key <key> "key file path").required(false))
    .arg(arg!(-c --create "create key if not exist"))
    .arg(arg!([fp] "file path"));

  let m = cmd.clone().get_matches();

  if let Some::<&String>(fp) = m.get_one("fp") {
    cget!(
      m:
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
  } else {
    cmd.print_long_help()?;
  }
  OK
}
