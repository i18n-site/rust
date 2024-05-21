#![feature(iter_array_chunks)]
#![feature(let_chains)]

genv::s!(SQL_BATCH_SIZE:usize | 1024);

pub const DOT_V: &str = ".v";
pub type HashLen = ([u8; 32], usize);

include!(concat!(env!("OUT_DIR"), "/INIT_SQL.rs"));

mod upload;
pub use upload::Upload;

mod pg;
pub use pg::Pg;

mod s3;
pub use s3::S3;

mod ext_url_li_build;
pub use ext_url_li_build::ExtUrlLiBuild;

mod err;
pub use err::Err;

mod conf;
pub use conf::Conf;

pub mod api;

mod vdir;
pub use vdir::VDir;

macro_rules! r#mod {
  ($($name:ident),*) => {
    $(
      mod $name;
      pub use $name::$name;
    )*
  }
}

r#mod!(
  cli,
  run,
  gen,
  nav_i18n,
  yml_li_push,
  mime,
  s3_upload,
  s3_hash_id,
  js,
  foot_pug,
  css_js
);

pub use yml_li_push::yml_li_lpush;
