mod parse;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use parse::Parser;
use parse::ParseError;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).unwrap();

    let mut b = String::new();
    f.read_to_string(&mut b).unwrap();
    let i = &mut b.chars().peekable();
    let mut parser = Parser::new(i);
    match parser.compilation_unit() {
        Ok(s) => println!("Ok! {:?}", s),
        Err(e) => println!("parse failed: {}", e.msg)
    }
}
