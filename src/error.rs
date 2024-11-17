use derive_more::derive::{Display, Error, From};
use embassy_rp::adc;

#[allow(dead_code, reason = "because")]
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq, reason = "because")]
#[derive(Clone, Debug, Display, Error, From, PartialEq)]
pub enum Error {
    #[display("_0")]
    Adc(#[error(not(source))] adc::Error),
}
