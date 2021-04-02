mod eval;
mod define_var;

use std::collections::HashMap;

pub use eval::eval;
use pipoquinha::atom::Atom;

pub type VarTable = HashMap<String, Atom>;
