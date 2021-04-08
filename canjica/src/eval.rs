mod arithmetic;
mod boolean;
mod comparison;
mod definition;
mod function;
mod io;
mod list;
mod special;
mod string;
mod vector;

use pipoquinha::parser::{
  atom::Atom::{self, *},
  identifier::is_builtin,
};

use super::VarTable;

pub fn eval(atom: Atom, variables: &VarTable) -> Atom {
  match atom {
    Identifier(id) if is_builtin(id.as_str()) => Identifier(id),
    Identifier(id) => {
      if let Some(value) = variables.get(id.as_str()) {
        value.clone()
      } else {
        Error(format!("Undefined variable: {}", id))
      }
    }
    List(l) => list::execute(*l, variables),
    UnappliedList(l) => List(l),
    Vector(l) => Vector(l.into_iter().map(|item| eval(item, variables)).collect()),
    f @ Function(_) => f,
    n @ Number(_) => n,
    b @ Bool(_) => b,
    e @ Error(_) => e,
    v @ VariableInsertion(_, _) => v,
    s @ Str(_) => s,
    Nil => Nil,
  }
}
