#![feature(let_chains)]

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

r#mod!(cli, run, gen, nav_li);
