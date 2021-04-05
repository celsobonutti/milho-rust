mod eval;

use std::collections::HashMap;

pub use eval::eval;
use pipoquinha::parser::atom::Atom;

pub type VarTable = HashMap<String, Atom>;
