use std::cell::RefCell;
use std::fmt;
use parse::SExp;
use symtable::SymbolRef;
use symtable::SymTable;
use std::collections::HashMap;
use std::slice::Iter;
use std::rc::Rc;

#[derive(Clone)]
enum Fun {
    Native{name:SymbolRef, code:Rc<Box<Fn(&[Data])->Data>>},
    User{params: Vec<SymbolRef>, body: SExp}
}

impl fmt::Debug for Fun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Fun::Native{ref name, ..} => write!(f, "Fun::Native {{ name: {:?}, code: .. }}", name),
            &Fun::User{..} => write!(f, "Fun::User {{ .. }}"),
        }
    }
}

impl PartialEq for Fun {
    fn eq(&self, other: &Fun) -> bool {
        match (self, other) {
            (&Fun::Native{ref name, ref code}, &Fun::Native{name:ref othername, code:ref othercode}) => {
                let a = code.as_ref() as *const _;
                let b = othercode.as_ref() as *const _;
                a == b
            },
            (&Fun::User{ref params, ref body}, &Fun::User{params:ref otherparams, body:ref otherbody}) => {
                params == otherparams && body == otherbody
            },
            _ => false,
        }
    }
}
impl Eq for Fun {
}

impl Fun {
    fn apply(&self, interp:&Interpreter, args: &[Data]) -> Data {
        match *self {
            Fun::Native{ref name, ref code} => (code)(args),
            Fun::User{ref params, ref body} => interp.user_call(args, params, body),
        }
    }
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub enum Data {
    DExp(SExp),
    DFun(Fun),
}

struct Env {
    parent: Option<Rc<Env>>,  // TODO: try removing Rc in favor of &, and a mess of explicit lifetime annotations
    vars: RefCell<HashMap<SymbolRef, Data>>,
}

impl Env {
    fn new(parent: Option<Rc<Env>>) -> Env {
        Env {
            parent: parent,
            vars: RefCell::new(HashMap::new()),
        }
    }
}

pub struct Interpreter {
    env: RefCell<Rc<Env>>,
    sym_if: SymbolRef,
    sym_lambda: SymbolRef,
    sym_let: SymbolRef,
}

impl Interpreter {
    pub fn new(st: &SymTable) -> Interpreter {
        Interpreter {
            env: RefCell::new(Rc::new(Env::new(None))),
            sym_if: st.sym_for("if"),
            sym_lambda: st.sym_for("lambda"),
            sym_let: st.sym_for("let"),
        }
    }

    pub fn eval(&self, s:&Data) -> Data {
        match s {
            &Data::DExp(ref s) => self.eval_sexp(s),
            _ => panic!("not able to evaluate {:?}", s)
        }
    }

    pub fn eval_expressions(&self, l:&Vec<SExp>) -> Data {
        l.iter().fold(Data::DExp(SExp::Nil), |_, x| self.eval_sexp(x))
    }

    pub fn eval_sexp(&self, s:&SExp) -> Data {
        match s {
            &SExp::List(ref l) => self.list(l),
            &SExp::Num(ref n) => Data::DExp(SExp::Num(*n)),
            &SExp::LString(ref s) => Data::DExp(SExp::LString(s.clone())),
            &SExp::Boolean(ref b) => Data::DExp(SExp::Boolean(*b)),
            &SExp::Sym(ref b) => self.lookup(b),
            _ => panic!("not able to evaluate {:?}", s)
        }
    }

    fn list(&self, l:&Vec<SExp>) -> Data {
        let mut args = l.iter();
        match args.next() {
            None => panic!("tried to invoke empty list {:?}", l),
            Some(&SExp::Sym(ref s)) => {
                if s == &self.sym_if {
                    self.form_if(args)
                } else if s == &self.sym_lambda {
                    self.form_lambda(args)
                } else if s == &self.sym_let {
                    self.form_let(args)
                } else {
                    self.apply(s, args)
                }
            },
            Some(other @ _) => panic!("expected symbol, found {:?}", other)
        }
    }

    fn apply(&self, s: &SymbolRef, args: Iter<SExp>) -> Data {
        let mut env = Some(self.env.borrow().clone());
        while let Some(envref) = env {
            match envref.vars.borrow().get(s) {
                Some(&Data::DFun(ref f)) => {
                    let vals = args.map(|a| self.eval_sexp(a) ).collect::<Vec<Data>>();
                    return f.apply(&self, &vals);
                },
                Some(v) => panic!("not a function: {:?}", v),
                None => {
                    env = envref.parent.clone();
                }
            }
        };
        panic!("undefined {:?}", s);
    }

    fn lookup(&self, s: &SymbolRef) -> Data {
        let mut env = Some(self.env.borrow().clone());
        while let Some(envref) = env {
            match envref.vars.borrow().get(s) {
                Some(d) => {
                    return (*d).clone();
                },
                None => {
                    env = envref.parent.clone();
                }
            }
        };
        panic!("undefined {:?}", s);
    }

    fn form_if(&self, mut args: Iter<SExp>) -> Data {
        match args.next() {
            None => panic!("missing condition expression in 'if'"),
            Some(e) => {
                match self.eval_sexp(e) {
                    Data::DExp(SExp::Boolean(true)) => {
                        match args.next() {
                            Some(a) => self.eval_sexp(a),
                            None => panic!("too few values for 'if' expression"),
                        }
                    },
                    Data::DExp(SExp::Boolean(false)) => {
                        args.next();  // skip
                        match args.next() {
                            Some(a) => self.eval_sexp(a),
                            None => Data::DExp(SExp::Nil),
                        }
                    },
                    _ => panic!("'if' condition must be a boolean value")
                }
            }
        }
    }

    fn form_lambda(&self, mut args: Iter<SExp>) -> Data {
        let mut param_syms:Vec<SymbolRef> = vec!();
        match args.next() {
            None => panic!("missing argment list and function body in 'lambda'"),
            Some(&SExp::List(ref params)) => {
                for p in params {
                    match p {
                        &SExp::Sym(ref s) => param_syms.push(s.clone()),
                        e => panic!("'lambda' param list entries must be symbols: {:?}", e)
                    }
                }
                match args.next() {
                    None => panic!("missing function body in 'lambda'"),
                    Some(s) => {
                        Data::DFun(Fun::User{params:param_syms, body: s.clone()})
                    }
                }
            },
            Some(s) => panic!("'lambda' definition requires an argument list: {:?}", s)
        }
    }

    fn form_let(&self, mut args: Iter<SExp>) -> Data {
        match args.next() {
            None => panic!("missing variable name in 'let'"),
            Some(&SExp::Sym(ref name)) => {
                match args.next() {
                    None => panic!("missing variable value in 'let'"),
                    Some(s) => {
                        let val = self.eval_sexp(s);
                        let env = self.env.borrow();
                        env.vars.borrow_mut().insert(name.clone(), val);
                        Data::DExp(SExp::Nil)
                    }
                }
            }
            s => panic!("let variable name must be a symbol, got: {:?}", s)
        }
    }

    fn user_call(&self, args: &[Data], params: &[SymbolRef], body: &SExp) -> Data {
        let env_old = self.env.borrow().clone();
        let new_env = Rc::new(Env::new(Some(env_old.clone())));
        {
            let mut h = new_env.vars.borrow_mut();
            for (p, a) in params.iter().zip(args) {
                h.insert(p.clone(), a.clone());
            }
            println!("args: {:?}", h);
        }
        *self.env.borrow_mut() = new_env;
        let result = self.eval_sexp(body);
        *self.env.borrow_mut() = env_old;
        result
    }

    pub fn define_native<CB: 'static + Fn(&[Data])->Data>(&self, name: SymbolRef, c: CB) {
        let env = self.env.borrow();
        env.vars.borrow_mut().insert(name.clone(), Data::DFun(Fun::Native{name: name, code: Rc::new(Box::new(c))}));
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use builtin;
    use super::Data;
    use symtable::SymTable;
    use parse::SExp;
    use parse::Parser;
    use std::rc::Rc;
    use std::cell::Cell;

    #[test]
    fn fun_call() {
        let st = SymTable::new();
        let i = Interpreter::new(&st);
        let myfun_sym = st.sym_for("myfun");
        let call_args = vec!(SExp::Sym(myfun_sym.clone()), SExp::LString("hello".to_string()));
        let expected_args = vec!(Data::DExp(SExp::LString("hello".to_string())));
        let call = SExp::List(call_args);
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();
        i.define_native(myfun_sym, move |args:&[Data]| {
            assert_eq!(expected_args, args);
            called_clone.set(true);
            Data::DExp(SExp::Nil)
        });
        i.eval_sexp(&call);
        assert!(called.get());
    }

    #[test]
    fn fun_call_user() {
        let text = "(let succ (lambda (x) (plus x 1)))
                    (succ 1)";
        let st = SymTable::new();
        let i = Interpreter::new(&st);
        builtin::init(&st, &i);
        let mut parse = Parser::new(st, text.chars().peekable());
        let code = parse.compilation_unit();
        if let Ok(SExp::List(l)) = code {
            let result = i.eval_expressions(&l);
            assert_eq!(Data::DExp(SExp::Num(2)), result);
        } else {
            panic!("unexpected parse result {:?}", code);
        }
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
        assert_eq!(Data::DExp(SExp::Num(1)), interpreter.eval_sexp(&code))
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
        assert_eq!(Data::DExp(SExp::Num(2)), interpreter.eval_sexp(&code))
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
        assert_eq!(Data::DExp(SExp::Nil), interpreter.eval_sexp(&code))
    }
}
