use std::error::Error;
use std::fmt;
use log;

#[derive(Debug)]
pub enum UpgradeError {
    InvalidMethod,
    InvalidPath,
    InvalidVersion,
    InvalidValue,
    MissingHeader,
    ParseError(httparse::Error),
    Unknown,
}

impl fmt::Display for UpgradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occured while processing Upgrade Request")\
    }
}

impl Error for UpgradeError {}
