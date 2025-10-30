#![cfg_attr(docsrs, feature(doc_cfg))]

mod kv;
use kv::Kv;
pub mod layout;
use logforth::filter::env_filter::EnvFilterBuilder;
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
  use logforth::append;
  logforth::starter_log::builder()
    .dispatch(|d| {
      let d = d.filter(EnvFilterBuilder::from_default_env().build());
      #[cfg(feature = "stdout")]
      d.append(append::Stdout::default().with_layout(layout::Text::default()))
    })
    .apply();
}
