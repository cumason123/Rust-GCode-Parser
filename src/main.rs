// #[macro_use]
// extern crate nom;
// extern crate nalgebra;

// mod parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn parse_file<'a>(filename: &str) -> Result<u32, &'static str> {
    let f = File::open(filename);

    let f = match f {
        Ok(file) => file,
        Err(_err) => {
            let phrase: &str = "Invalid file. Check integrity of {}";
            let error_message: &'static str = &format!("{}{}", phrase, filename);
            return Err(error_message);
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

fn main() {
    parse_file("tort.ngc");
    println!("Finished");
}
