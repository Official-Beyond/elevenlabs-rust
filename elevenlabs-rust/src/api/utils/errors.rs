use std::{fmt, error};

#[derive(Debug)]
pub enum UtilsError {
    Http(reqwest::Error),
    Io(std::io::Error),
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UtilsError::Http(ref err) => write!(f, "HTTP Error: {}", err),
            UtilsError::Io(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl error::Error for UtilsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            UtilsError::Http(ref err) => Some(err),
            UtilsError::Io(ref err) => Some(err),
        }
    }
}
