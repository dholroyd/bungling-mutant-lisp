use std::cell::RefCell;
use parse::SExp;
use symtable::SymbolRef;
use std::collections::HashMap;


pub struct Interpreter {
    env:RefCell<HashMap<SymbolRef, Fun>>,
}

enum Fun {
    Native{name:SymbolRef, code:Box<Fn(&[SExp])>},
    User(SExp)
}

impl Fun {
    fn apply(&self, args: &[SExp]) {
        match *self {
            Fun::Native{ref name, ref code} => (code)(args),
            Fun::User(ref exp) => panic!("user functons unimplemented")
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(HashMap::new()),
        }
    }

    pub fn start(&self, s:SExp) {
        match s {
            SExp::List(l) => self.apply(&l),
            _ => println!("unexpected {:?}", s)
        }
    }

    fn apply(&self, l:&Vec<SExp>) {
        let mut args = l.iter();
        match args.next() {
            None                => panic!("tried to invoke empty list {:?}", l),
            Some(&SExp::Sym(ref s)) => {
                println!("would apply {:?}", s);
                match self.env.borrow().get(s) {
                    None => panic!("function not defined: {:?}", s),
                    Some(f) => f.apply(l)
                }
            },
            Some(other @ _) => panic!("expected symbol, found {:?}", other)
        }
    }

    pub fn define_native<CB: 'static + Fn(&[SExp])>(&self, name: SymbolRef, c: CB) {
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
        let i = Interpreter::new();
        let myfun_sym = st.sym_for("myfun");
        let call_args = vec!(SExp::Sym(myfun_sym.clone()), SExp::LString("hello".to_string()));
        let expected_args = vec!(SExp::Sym(myfun_sym.clone()), SExp::LString("hello".to_string()));
        let call = SExp::List(call_args);
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();
        i.define_native(myfun_sym, move |args:&[SExp]| {
            assert_eq!(expected_args, args);
            called_clone.set(true);
        });
        i.start(call);
        assert!(called.get());
    }

}
