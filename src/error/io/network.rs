use crate::shared_consts::*;
use thiserror::Error;

#[allow(dead_code)]
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq, clippy::enum_variant_names)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {}