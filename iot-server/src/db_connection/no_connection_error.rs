use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct NoConnectionError;

impl Display for NoConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No connection to server!")
    }
}

impl Error for NoConnectionError {}
