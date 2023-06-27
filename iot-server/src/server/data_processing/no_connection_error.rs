use std::fmt::Display;
use std::{error::Error};


#[derive(Debug)]
pub struct NoConnectionError;

impl Display for NoConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong")
    }
}

impl Error for NoConnectionError {}