[‼️]: ✏️README.mdt

# cget

```rust
use std::process::exit;

use aok::{Result, OK};
use cget::cget;
use clap::{arg, crate_version, Command};
//use current_platform::CURRENT_PLATFORM;

fn main() -> Result<()> {
  let m = Command::new("hsc")
    .disable_version_flag(true)
    .arg(arg!(-c --create "create key if not exist"))
    .arg(arg!(-k --key <key> "key file path"))
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

  //   if m.get_one("vv") == Some(&true) {
  //     println!(
  //       r#"ver:{}
  // build_target:{}"#,
  //       crate_version!(),
  //       CURRENT_PLATFORM
  //     );
  //     exit(0);
  //   }

  cget!(
    m:
      fp: String;
      key: String;
      create: bool;
  );

  println!("{} {} {}", fp, key, create);

  OK
}
```
