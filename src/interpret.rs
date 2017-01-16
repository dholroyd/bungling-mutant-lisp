use std::cell::RefCell;
use parse::SExp;
use symtable::SymbolRef;
use symtable::SymTable;
use std::collections::HashMap;
use std::slice::Iter;

pub struct Interpreter {
    env:RefCell<HashMap<SymbolRef, Fun>>,
    sym_if: SymbolRef,
}

enum Fun {
    Native{name:SymbolRef, code:Box<Fn(&[SExp])->SExp>},
    User(SExp)
}

impl Fun {
    fn apply(&self, args: &[SExp]) -> SExp {
        match *self {
            Fun::Native{ref name, ref code} => (code)(args),
            Fun::User(ref exp) => panic!("user functons unimplemented")
        }
    }
}

impl Interpreter {
    pub fn new(st: &SymTable) -> Interpreter {
        Interpreter {
            env: RefCell::new(HashMap::new()),
            sym_if: st.sym_for("if"),
        }
    }

    pub fn eval(&self, s:&SExp) -> SExp {
        match s {
            &SExp::List(ref l) => self.list(l),
            &SExp::Num(ref n) => SExp::Num(*n),
            &SExp::LString(ref s) => SExp::LString(s.clone()),
            &SExp::Boolean(ref b) => SExp::Boolean(*b),
            _ => panic!("not able to evaluate {:?}", s)
        }
    }

    fn list(&self, l:&Vec<SExp>) -> SExp {
        let mut args = l.iter();
        match args.next() {
            None => panic!("tried to invoke empty list {:?}", l),
            Some(&SExp::Sym(ref s)) => {
                if s == &self.sym_if {
                    self.form_if(args)
                } else {
                    self.apply(s, args)
                }
            },
            Some(other @ _) => panic!("expected symbol, found {:?}", other)
        }
    }

    fn apply(&self, s: &SymbolRef, args: Iter<SExp>) -> SExp {
        match self.env.borrow().get(s) {
            None => panic!("function not defined: {:?}", s),
            Some(f) => {
                let vals = args.map(|a| self.eval(a) ).collect::<Vec<SExp>>();
                println!("apply {:?}", s);
                f.apply(&vals)
            }
        }
    }

    fn form_if(&self, mut args: Iter<SExp>) -> SExp {
        match args.next() {
            None => panic!("missing condition expression in 'if'"),
            Some(e) => {
                match self.eval(e) {
                    SExp::Boolean(true) => {
                        match args.next() {
                            Some(a) => self.eval(a),
                            None => panic!("too few values for 'if' expression"),
                        }
                    },
                    SExp::Boolean(false) => {
                        args.next();  // skip
                        match args.next() {
                            Some(a) => self.eval(a),
                            None => SExp::Nil,
                        }
                    },
                    _ => panic!("'if' condition must be a boolean value")
                }
            }
        }
    }

    pub fn define_native<CB: 'static + Fn(&[SExp])->SExp>(&self, name: SymbolRef, c: CB) {
        self.env.borrow_mut().insert(name.clone(), Fun::Native{name: name, code: Box::new(c)});
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use symtable::SymTable;
    use parse::SExp;
    use std::rc::Rc;
    use std::cell::Cell;

    #[test]
    fn fun_call() {
        let st = SymTable::new();
        let i = Interpreter::new(&st);
        let myfun_sym = st.sym_for("myfun");
        let call_args = vec!(SExp::Sym(myfun_sym.clone()), SExp::LString("hello".to_string()));
        let expected_args = vec!(SExp::LString("hello".to_string()));
        let call = SExp::List(call_args);
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();
        i.define_native(myfun_sym, move |args:&[SExp]| {
            assert_eq!(expected_args, args);
            called_clone.set(true);
            SExp::Nil
        });
        i.eval(&call);
        assert!(called.get());
    }

    #[test]
    fn ifelse_true() {
        let st = SymTable::new();
        let interpreter = Interpreter::new(&st);
        let code = SExp::List(vec!(
            SExp::Sym(st.sym_for("if")),
            SExp::Boolean(true),
            SExp::Num(1),
            SExp::Num(2),
        ));
        assert_eq!(SExp::Num(1), interpreter.eval(&code))
    }

    #[test]
    fn ifelse_false() {
        let st = SymTable::new();
        let interpreter = Interpreter::new(&st);
        let code = SExp::List(vec!(
            SExp::Sym(st.sym_for("if")),
            SExp::Boolean(false),
            SExp::Num(1),
            SExp::Num(2),
        ));
        assert_eq!(SExp::Num(2), interpreter.eval(&code))
    }

    #[test]
    fn if_false() {
        let st = SymTable::new();
        let interpreter = Interpreter::new(&st);
        let code = SExp::List(vec!(
            SExp::Sym(st.sym_for("if")),
            SExp::Boolean(false),
            SExp::Num(1),
        ));
        assert_eq!(SExp::Nil, interpreter.eval(&code))
    }
}
