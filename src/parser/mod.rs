pub mod parsing_error;
pub mod lexicon;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use self::parsing_error::ParsingError;

pub fn parse_file(filename: &str) -> Result<u32, ParsingError> {
    let f = File::open(filename);

    let f = match f {
        Ok(file) => file,
        Err(err) => {
            return Err(ParsingError::new(&format!("Could not open {}", filename)));
        }
    };

    let mut file = BufReader::new(&f);
    let mut line = String::new();
    let mut time = 0u32;

    while file.read_line(&mut line).unwrap() > 0 {
        // println!("{}", line);
    }

    Ok(time)
}

fn parse_line(line: &str) {

}
