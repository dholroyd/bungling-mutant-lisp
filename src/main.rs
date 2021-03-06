mod parse;
mod symtable;
mod interpret;
mod builtin;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use parse::Parser;
use symtable::SymTable;
use parse::SExp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).unwrap();

    let mut b = String::new();
    f.read_to_string(&mut b).unwrap();
    let i = b.chars().peekable();
    let st = SymTable::new();
    let interpreter = interpret::Interpreter::new(&st);
    builtin::init(&st, &interpreter);
    let mut parser = Parser::new(st, i);
    match parser.compilation_unit() {
        Ok(SExp::List(l)) => {
            print!("end: {:?}", interpreter.eval_expressions(&l));
        },
        Ok(s) => {
            print!("end: {:?}", interpreter.eval(&interpret::Data::DExp(s)));
        },
        Err(e) => println!("parse failed: {}", e.msg)
    }
}
