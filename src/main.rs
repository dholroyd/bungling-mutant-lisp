mod parse;
mod symtable;
mod interpret;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use parse::Parser;
use parse::ParseError;
use parse::SExp;
use symtable::SymTable;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).unwrap();

    let mut b = String::new();
    f.read_to_string(&mut b).unwrap();
    let i = b.chars().peekable();
    let st = SymTable::new();
    let println_sym = st.sym_for("println");
    let mut parser = Parser::new(st, i);
    match parser.compilation_unit() {
        Ok(s) => {
            let interpreter = interpret::Interpreter::new();
            interpreter.define_native(println_sym, |args:&[SExp]| println!("println: {:?}", args) );
            interpreter.start(s)
        },
        Err(e) => println!("parse failed: {}", e.msg)
    }
}
