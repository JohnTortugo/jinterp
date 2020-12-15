#![allow(unused)]
#![allow(non_camel_case_types)]

use std::env;
use std::fs::File;
use std::io::BufReader;
mod spec;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Parsing {}", filename);

    let mut file = File::open(filename).expect("file not found.");
    let mut buf_reader = BufReader::new(file);

    let java_class = spec::ClassFile::load(&mut buf_reader);

    java_class.print(true, false, false, true, true);
}
