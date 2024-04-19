#![feature(const_trait_impl)]
#![feature(let_chains)]
#![feature(effects)]

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

pub const EMPTY: String = String::new();

pub struct Site {
  pub host: String,
  pub route_li: Vec<String>,
  pub nav_li: Vec<String>,
  pub channel: String,
  pub ver: String,
}

pub mod upload;
pub use upload::Upload;

mod err;
pub use err::Err;

pub mod conf;
pub use conf::Conf;

mod cli;
pub use cli::cli;

mod run;
pub use run::run;

mod site_lang;
pub use site_lang::site_lang;
