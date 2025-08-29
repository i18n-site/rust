#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

mod kv;
use kv::Kv;
pub mod layout;
use logforth::filter::EnvFilter;
use workspace_root::get_workspace_root;

#[static_init::dynamic]
pub static TZ: jiff::tz::TimeZone = jiff::tz::TimeZone::try_system().unwrap();

#[static_init::dynamic]
pub static ROOT: String = get_workspace_root().to_str().unwrap().to_owned();

#[static_init::dynamic]
pub static HOME_DIR: String = dirs::home_dir().unwrap().to_str().unwrap().to_owned();

pub fn init() {
  use logforth::append;
  logforth::builder()
    .dispatch(|d| {
      let d = d.filter(EnvFilter::from_default_env());
      #[cfg(feature = "stdout")]
      d.append(append::Stdout::default().with_layout(layout::Text::default()))
    })
    .apply();
}
