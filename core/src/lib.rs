pub mod models;
pub mod db;
pub mod migrator;
pub mod error;
pub mod constants;
pub mod writers;
pub mod parsers;
pub mod utils;

pub use crate::error::DMError;

pub type DMResult<T> = Result<T, DMError>;