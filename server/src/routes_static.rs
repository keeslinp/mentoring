use std::env::{var, VarError};
use std::path::PathBuf;

use staticfile::Static;

use errors::{Error, ErrorKind, Result};

pub fn handler() -> Result<Static> {
    let path = match var("STATIC_PATH") {
        Ok(val) => PathBuf::from(val),
        Err(VarError::NotPresent) => PathBuf::from("static"),
        Err(e) => return Err(Error::with_chain(e, ErrorKind::InvalidEnvVar("STATIC_PATH"))),
    };
    Ok(Static::new(path))
}
