#![feature(let_chains)]

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
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
