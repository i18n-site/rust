#![feature(let_chains)]

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
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  if let Some((m, mut cmd)) = cmdv!(
    arg!(-k --key <key> "key file path").required(false),
    arg!(-c --create "create key if not exist"),
    arg!([fp] "file path")
  ) {
    if let Some::<&String>(fp) = m.get_one("fp") {
      cget!(
        m:
          create: bool;
      );
      let key = if let Some(key) = m.get_one::<String>("key") {
        hsc::key(key, *create).await?
      } else {
        let sk: String = B3S_SK();
        let key = &BASE64_STANDARD.decode(sk)?[..];
        SigningKey::from_bytes(&key.try_into()?)
      };

      hsc::hsc(fp, key).await?;
    } else {
      cmd.print_help()?;
    }
  }
  OK
}
