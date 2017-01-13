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
            panic!("expected 2 arguments, {:?}", args)
        }
        match (&args[0], &args[1]) {
            (&SExp::Num(a), &SExp::Num(b)) => SExp::Num(a+b),
            _ => panic!("invalid arguments for 'plus'")
        }
    });
}
