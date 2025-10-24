pub mod config;
pub mod error;
pub mod network;
pub mod protocol;

pub use error::HysteriaError;
pub use network::{DuplexStream, HysteriaClient, connect};

pub type Result<T> = std::result::Result<T, HysteriaError>;
