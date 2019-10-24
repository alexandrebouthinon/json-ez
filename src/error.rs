use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    NotFound(String, String),
    CannotConvert(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::NotFound(k, o) => f.write_str(&format!(
                "NotFound: Cannot found key {} in {}",
                k, o
            )),
            Error::CannotConvert(v) => f.write_str(&format!(
                "CannotConvert: Cannot convert value {} to desired type",
                v
            )),
        }
    }
}
