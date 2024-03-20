pub use anyhow::*;

pub const OK: std::result::Result<(), Error> = std::result::Result::Ok(());
pub type Null = anyhow::Result<()>;
