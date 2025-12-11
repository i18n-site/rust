#![cfg_attr(docsrs, feature(doc_cfg))]

mod kv;
pub mod layout;

use std::env;

use kv::Kv;
use logforth::{append, filter::env_filter::EnvFilterBuilder};
use logforth_append_file::FileBuilder;
use path_end::path_end;
use workspace_root::get_workspace_root_directory;

#[static_init::dynamic]
pub static TZ: jiff::tz::TimeZone = jiff::tz::TimeZone::try_system().unwrap();

#[static_init::dynamic]
pub static ROOT: String = {
  path_end(
    get_workspace_root_directory()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default(),
  )
};

#[static_init::dynamic]
pub static HOME_DIR: String = path_end(
  dirs::home_dir()
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default(),
);

pub fn init() {
  let stdout = || {
    #[cfg(feature = "stdout")]
    {
      append::Stdout::default().with_layout(layout::Text::default())
    }
    #[cfg(not(feature = "stdout"))]
    {
      panic!("No stdout feature enabled and no file logging available")
    }
  };

  logforth::starter_log::builder()
    .dispatch(|d| {
      let d = d.filter(EnvFilterBuilder::from_default_env().build());

      match env::var("LOGS_DIRECTORY") {
        Ok(dir) => {
          let name = format!("{}.log", exe_name());
          let _ = std::fs::create_dir_all(&dir);

          match create_file(&dir, &name) {
            Ok(app) => d.append(app),
            Err(e) => {
              eprintln!("logging file failed: {}, using stdout", e);
              d.append(stdout())
            }
          }
        }
        Err(_) => d.append(stdout()),
      }
    })
    .apply();
}

fn create_file(dir: &str, name: &str) -> Result<impl logforth::Append, Box<dyn std::error::Error>> {
  let app = FileBuilder::new(dir, name)
    .layout(layout::Text::default())
    .build()?;
  Ok(app)
}

fn exe_name() -> String {
  env::current_exe()
    .ok()
    .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
    .unwrap_or_else(|| "app".to_string())
}
