use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};
use typed_arena::Arena as TypedArena;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Symbol(usize);

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Symbol {
    pub fn as_str(self) -> &'static str {
        with_interner(|interner| interner.get_str(self))
    }

    pub fn intern(s: &str) -> Symbol {
        with_interner(|interner| interner.intern(s))
    }
}

thread_local! {
    pub static INTERNER: RefCell<Interner> = Default::default();
}

fn with_interner<R>(f: impl FnOnce(&mut Interner) -> R) -> R {
    INTERNER.with(|cell| f(&mut cell.borrow_mut()))
}

#[derive(Default)]
pub struct Interner {
    symbols: FxHashMap<&'static str, Symbol>,
    strs: Vec<&'static str>,
    arena: TypedArena<String>,
}

impl Interner {
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&sym) = self.symbols.get(s) {
            return sym;
        }
        let s: &str = &*self.arena.alloc(s.to_owned());
        // SAFETY: will only access strings while interner/arena is alive
        let s: &'static str = unsafe { &*(s as *const str) };
        let symbol = Symbol(self.strs.len());
        self.strs.push(s);
        self.symbols.insert(s, symbol);
        symbol
    }

    pub fn get_str(&self, symbol: Symbol) -> &'static str {
        self.strs[symbol.0]
    }
}
