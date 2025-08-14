#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

#[cfg(feature = "is_cn_char")]
pub mod is_cn_char;
#[cfg(feature = "is_cn_char")]
pub use is_cn_char::is_cn_char;
