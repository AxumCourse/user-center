pub mod config;
pub mod db;
mod errs;
pub mod handler;
pub mod model;
pub mod utils;

pub use errs::{Error, ErrorKind};

pub type Result<T> = std::result::Result<T, crate::Error>;
