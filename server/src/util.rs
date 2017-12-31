use std::env::{var, VarError};
use std::error::Error as StdError;
use std::str::FromStr;

use errors::{Error, ErrorKind, Result, ResultExt};

pub fn getenv(name: &'static str) -> Result<Option<String>> {
    match var(name) {
        Ok(val) => Ok(Some(val)),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => Err(Error::with_chain(e, ErrorKind::InvalidEnvVar(name))),
    }
}

pub fn getenv_parse<T>(name: &'static str) -> Result<Option<T>>
where
    T: FromStr,
    T::Err: 'static + Send + StdError,
{
    match getenv(name) {
        Ok(Some(val)) => val.parse()
            .chain_err(|| ErrorKind::InvalidEnvVar(name))
            .map(Some),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}
