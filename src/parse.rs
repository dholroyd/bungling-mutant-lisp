use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
enum SExp {
    Sym(String),
    List(Vec<SExp>)
}

pub struct ParseError {
    pub msg:String
}

type ParseResult = Result<SExp, ParseError>;

pub struct Parser<'a> {
    i:&'a mut Peekable<Chars<'a>>
}

fn esc(c:char) -> String {
    match c {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        _ => c.to_string()
    }
}

impl<'a> Parser<'a> {

    pub fn new(i:&'a mut Peekable<Chars<'a>>) -> Parser<'a> {
        Parser{i: i}
    }

    fn skip_ws(&mut self) {
        loop {
            match self.i.peek() {
                None => break,
                Some(&' ') => (),
                Some(&'\n') => (),
                Some(&'\r') => (),
                Some(&'\t') => (),
                Some(_) => break
            }
            let c = self.i.next().unwrap();
            println!("skipped <{}>", esc(c));
        }
    }

    fn skip_comment(&mut self) {
        if self.i.peek().unwrap() == &';' {  // TODO None
            loop {
                match self.i.next() {
                    None => break,
                    Some('\n') => break,
                    Some(_) => ()
                }
            }
        }
    }

    fn expect(&mut self, e:char) {
        let c = self.i.next().unwrap();
        if c != e {  // TODO None
            panic!("expected '{}', got '{}'", esc(e), esc(c));
        }
        println!("found expected <{}>", esc(c));
    }

    fn peek(&mut self, e:char) -> bool {
        self.i.peek().map_or(false, |v| v == &e)
    }

    fn sym(&mut self) -> ParseResult {
        let mut s = String::new();
        loop {
            match self.i.peek() {
                None => break,
                Some(c @ &'a' ... 'z') => s.push(*c),
                Some(_) => break
            }
            self.i.next();
        }
        Ok(SExp::Sym(s))
    }

    fn atom(&mut self) -> ParseResult {
        let chr:char;
        match self.i.peek() {
            None => return Err(ParseError{msg:"end of input while expecting an ATOM".to_string()}),
            Some(c) => chr = *c
        }
        match chr {
            '('         => self.list(),
            'a'...'z'   => self.sym(),
            chr         => Err(ParseError{msg:format!("expected LIST or SYMBOL, but found '{}'", chr)})
        }
    }

    fn list(&mut self) -> ParseResult {
        self.expect('(');
        self.skip_ws();
        let mut v:Vec<SExp> = Vec::new();
        while !self.peek(')') {
            v.push(try!(self.atom()));
            self.skip_ws();
        }
        self.expect(')');
        Ok(SExp::List(v))
    }

    pub fn compilation_unit(&mut self) -> ParseResult {
        self.skip_ws();
        self.list()
    }

}
