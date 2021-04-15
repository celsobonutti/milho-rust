use pipoquinha::parser::unsafe_parse_atom;
use pipoquinha::types::Atom;

use crate::{eval, Table};

mod arithmetics;
mod comparison;

pub fn eval_with_empty_tables(input: &str) -> Atom {
  eval(unsafe_parse_atom(input), Table::initialize(vec![]))
}
