mod arithmetic;
mod comparison;
mod list;
mod definition;
mod function;

use pipoquinha::parser::atom::Atom::{self, *};

use super::VarTable;

pub fn eval(atom: Atom, variables: &VarTable) -> Atom {
  match atom {
    Identifier(id) => {
      if let Some(value) = variables.get(id.as_str()) {
        value.clone()
      } else {
        Error(format!("Undefined variable: {}", id))
      }
    }
    List(l) => list::execute(*l, variables),
    Vector(l) => Vector(l.into_iter().map(|item| eval(item, variables)).collect()),
    f @ Function(_) => f,
    n @ Number(_) => n,
    b @ Bool(_) => b,
    e @ Error(_) => e,
    v @ VariableInsertion(_,_) => v,
  }
}
