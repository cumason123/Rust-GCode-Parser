use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct InvalidFileError;

impl Error for InvalidFileError {
    fn description(&self) -> &str {
        "Invalid gcode file"
    }
}

impl fmt::Display for InvalidFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Please check the given file line's, or the integrity of the file")
    }
}
