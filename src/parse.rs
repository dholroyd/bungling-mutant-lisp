use std::cell::RefCell;
use std::str::Chars;
use std::iter::Peekable;
use symtable::SymTable;
use symtable::SymbolRef;

#[derive(Debug)]
pub enum SExp {
    Sym(SymbolRef),
    LString(String),
    List(Vec<SExp>)
}

pub struct ParseError {
    pub msg:String
}

type ParseResult<'a> = Result<SExp, ParseError>;

pub struct Parser<'a> {
    st:SymTable,
    i:RefCell<Peekable<Chars<'a>>>
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

    pub fn new(st:SymTable, i:Peekable<Chars<'a>>) -> Parser<'a> {
        Parser{
            st: st,
            i: RefCell::new(i)
        }
    }

    fn peek(&self) -> Option<char> {
            self.i.borrow_mut().peek().map(|c| *c)
    }

    fn next(&self) -> Option<char> {
            self.i.borrow_mut().next()
    }

    fn skip_ws(&self) {
        loop {
            match self.peek() {
                None => break,
                Some(' ') => (),
                Some('\n') => (),
                Some('\r') => (),
                Some('\t') => (),
                Some(_) => break
            }
            let c = self.next().unwrap();
            println!("skipped <{}>", esc(c));
        }
    }

    fn skip_comment(&self) {
        if self.peek_matches(';') {
            loop {
                match self.next() {
                    None => break,
                    Some('\n') => break,
                    Some(_) => ()
                }
            }
        }
    }

    fn expect(&self, e:char) {
        let c = self.next().unwrap();
        if c != e {  // TODO None
            panic!("expected '{}', got '{}'", esc(e), esc(c));
        }
        println!("found expected <{}>", esc(c));
    }

    fn peek_matches(&self, e:char) -> bool {
        self.peek().map_or(false, |v| v == e)
    }

    fn sym(&self) -> ParseResult {
        let mut s = String::new();
        loop {
            match self.peek() {
                None => break,
                Some(c @ 'a' ... 'z') => s.push(c),
                Some(_) => break
            }
            self.next();
        }
        Ok(SExp::Sym(self.st.sym_for(&s)))
    }

    fn string(&self) -> ParseResult {
        self.expect('"');
        let mut s = String::new();
        loop {
            let chr:char;
            match self.peek() {
                None => break,
                Some(c) => chr = c
            }
            match chr {
                '\\' => {
                    self.expect('\\');
                    match self.peek() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('r') => s.push('\r'),
                        Some('"') => s.push('"'),
                        Some('\\') => s.push('\\'),
                        Some(_) => return Err(ParseError{msg:"invalud escape sequence".to_string()}),
                        None => return Err(ParseError{msg:"end of input within string literal".to_string()})
                    }
                },
                '"' => break,
                _ => s.push(chr)
            }
            self.next();
        }
        self.expect('"');
        Ok(SExp::LString(s))
    }

    fn atom(&self) -> ParseResult {
        let chr:char;
        match self.peek() {
            None => return Err(ParseError{msg:"end of input while expecting an ATOM".to_string()}),
            Some(c) => chr = c
        }
        match chr {
            '('         => self.list(),
            '"'         => self.string(),
            'a'...'z'   => self.sym(),
            chr         => Err(ParseError{msg:format!("expected LIST, STRING or SYMBOL, but found '{}'", chr)})
        }
    }

    fn list(&self) -> ParseResult {
        self.expect('(');
        self.skip_ws();
        let mut v:Vec<SExp> = Vec::new();
        while !self.peek_matches(')') {
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
