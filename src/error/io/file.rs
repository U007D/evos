use crate::shared_consts::*;
use thiserror::Error;

#[allow(dead_code)]
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq, clippy::enum_variant_names)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{} {:?}", msg::ERR_FILE_CREATE, 0)]
    FileCreateError(&'static str),
    #[error("{} {:?}", msg::ERR_FILE_OPEN, 0)]
    FileOpenError(&'static str),
    #[error("{} {:?}", msg::ERR_FILE_READ, 0)]
    FileReadError(&'static str),
    #[error("{} {:?}", msg::ERR_FILE_WRITE, 0)]
    FileWriteError(&'static str),
}