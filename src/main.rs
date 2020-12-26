#![allow(unused)]
#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::BufReader;
use std::env;
mod utils;
mod spec;
mod bytecode;
mod attributes;
mod constantpool;
mod interpreter;

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("File not found.");
    let mut buf_reader = BufReader::new(file);
    let mut startup_class = spec::ClassDesc::new(&mut buf_reader);
    let mut interpreter = interpreter::Interpreter::new(&mut startup_class, filename);

    interpreter.run();
}
