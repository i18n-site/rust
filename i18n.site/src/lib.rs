#![feature(str_split_remainder)]
#![feature(iter_array_chunks)]

pub const DOT_V: &str = ".v";
pub type HashLen = ([u8; 32], usize);

macro_rules! r#mod {
  ($($name:ident),*) => {
    $(
      mod $name;
      pub use $name::$name;
    )*
  }
}

r#mod!(cli, run, package_json_ver, seo);
