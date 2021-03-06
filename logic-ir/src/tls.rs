use crate::{DebugCtxt, Interner, LogicInterner};
use std::cell::RefCell;

thread_local! {
    static DEBUG: RefCell<Option<LogicInterner>> = RefCell::new(None);
}

// pub fn set_debug_ctxt(debug: Box<dyn DebugCtxt>) {
//     DEBUG.with(|dbg| *dbg.borrow_mut() = Some(debug))
// }

// pub fn with_debug_ctxt<R>(f: impl FnOnce(&dyn DebugCtxt) -> R) -> R {
//     DEBUG.with(|dbg| f(dbg.borrow().as_ref().expect("no debug context set").as_ref()))
// }
