#![allow(unused)]
#![allow(non_camel_case_types)]

use std::env;
use std::fs::File;
use std::io::BufReader;
mod spec;
mod bytecode;
mod attributes;
mod constantpool;

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("File not found.");
    let mut buf_reader = BufReader::new(file);
    let java_class = spec::ClassFile::load(&mut buf_reader);

    java_class.print(true, false, false, true, true);
}
