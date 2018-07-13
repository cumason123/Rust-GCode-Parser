// #[macro_use]
// extern crate nom;
// extern crate nalgebra;
mod parser;
use parser::lexicon;
use parser::parsing_error;
use parser::parse_file;

fn main() {
    let res = parse_file("tort.ngc");
    println!("Finished");
}
