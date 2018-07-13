use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParsingError {
    details: String
}

impl ParsingError {
    pub fn new(msg: &str) -> ParsingError {
        ParsingError{details: msg.to_string()}
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.details
    }
}
