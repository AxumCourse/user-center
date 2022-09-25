pub mod config;
mod errs;
pub mod handler;
pub mod model;

pub use errs::{Error, ErrorKind};

pub type Result<T> = std::result::Result<T, crate::Error>;
