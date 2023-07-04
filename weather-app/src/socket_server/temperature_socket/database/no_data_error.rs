use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub struct NoDataError;


impl Display for NoDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No fitting data found in Database")
    }
}

impl Error for NoDataError {}