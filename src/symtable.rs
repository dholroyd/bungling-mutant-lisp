use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Symbol(String);

pub type SymbolRef = Rc<Symbol>;

pub struct SymTable {
    name_to_sym:RefCell<HashMap<String, SymbolRef>>,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self as *const _ == other as *const _
    }
}
impl Eq for Symbol {
}
impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self as *const _ as usize);
    }
}

impl SymTable {
    pub fn new() -> SymTable {
        SymTable {
            name_to_sym: RefCell::new(HashMap::new()),
        }
    }
    pub fn insert(&self, name:&str) -> Option<SymbolRef> {
        if self.name_to_sym.borrow().contains_key(name) {
            None
        } else {
            let sym = Rc::new(Symbol(name.to_string()));
            self.name_to_sym.borrow_mut().insert(name.to_string(), sym);
            self.name_to_sym.borrow().get(name).map(|s| s.clone() )
        }
    }

    pub fn sym_for(&self, name:&str) -> SymbolRef {
        match self.insert(name) {
            Some(s) => s,
            None => self.name_to_sym.borrow().get(name).unwrap().clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SymTable;

    #[test]
    fn it_works() {
        let st = SymTable::new();
        let foo = st.sym_for("foo");
        let bar = st.sym_for("bar");
        assert_eq!(foo, st.sym_for("foo"));
        assert_eq!(bar, st.sym_for("bar"));
        assert_ne!(foo, st.sym_for("bar"));

        let st2 = SymTable::new();
        assert_ne!(foo, st2.sym_for("foo"));
    }

}
