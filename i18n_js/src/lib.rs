#![feature(str_split_remainder)]
#![feature(const_trait_impl)]

mod index_html;
use index_html::index_html;
mod mnt;
pub use mnt::Mnt;
mod bjs_after;
pub use bjs_after::bjs_after;
mod i18n_li;
pub use i18n_li::I18nLi;
mod nav_li;
pub use nav_li::{Nav, NavLi};
mod build;
pub use build::Build;
mod conf;
pub use conf::{Conf, HtmConf};
mod css;
mod pug;
mod scan;
use scan::Scan;
mod js;
mod worker;

pub const EMPTY: String = String::new();
pub const GEN: &str = "gen";
pub const INDEX_PUG: &str = "index.pug";
pub const DOT_I18N: &str = ".i18n";
pub const HTM: &str = "htm";
pub const PUBLIC: &str = "public";
pub const OUT: &str = "out";
