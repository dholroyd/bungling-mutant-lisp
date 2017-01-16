use interpret::Interpreter;
use symtable::SymTable;
use parse::SExp;

pub fn init(st: &SymTable, interpreter: &Interpreter) {
    let println_sym = st.sym_for("println");
    interpreter.define_native(println_sym, |args:&[SExp]| {
        println!("println: {:?}", args);
        SExp::Nil
    });

    let plus_sym = st.sym_for("plus");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'plus' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Num(a+b),
            _ => panic!("invalid arguments for 'plus'")
        }
    });

    let plus_sym = st.sym_for("minus");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'minus' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Num(a-b),
            _ => panic!("invalid arguments for 'minus'")
        }
    });

    let plus_sym = st.sym_for("mul");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'mul' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Num(a*b),
            _ => panic!("invalid arguments for 'mul'")
        }
    });

    let plus_sym = st.sym_for("div");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'div' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Num(a/b),
            _ => panic!("invalid arguments for 'div'")
        }
    });

    let plus_sym = st.sym_for("lt");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'lt' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Boolean(a<b),
            _ => panic!("invalid arguments for 'lt'")
        }
    });

    let plus_sym = st.sym_for("le");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'le' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Boolean(a<=b),
            _ => panic!("invalid arguments for 'le'")
        }
    });

    let plus_sym = st.sym_for("gt");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'gt' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Boolean(a>b),
            _ => panic!("invalid arguments for 'gt'")
        }
    });

    let plus_sym = st.sym_for("ge");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 2 {
            panic!("'ge' expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Boolean(a>=b),
            _ => panic!("invalid arguments for 'ge'")
        }
    });

    let plus_sym = st.sym_for("not");
    interpreter.define_native(plus_sym, |args:&[SExp]| {
        if args.len() < 1 {
            panic!("'not' expected 1 argument")
        }
        match &args[0] {
            &SExp::Boolean(a) => SExp::Boolean(!a),
            _ => panic!("invalid arguments for 'not'")
        }
    });
}
