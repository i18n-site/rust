#![feature(min_specialization)]
mod err;
pub use err::{err, Err, Error, Result};
pub mod bad_request;
pub mod form;
mod msg;
pub use msg::{FnAny, Msg};
