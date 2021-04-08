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

use super::{NamespaceTable, VarTable};

pub fn eval(atom: Atom, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  match atom {
    Identifier(id) if is_builtin(id.as_str()) => Identifier(id),
    Identifier(id) => {
      if let Some(value) = local_variables.get(id.as_str()) {
        value.clone()
      } else if let Some(value) = namespace_variables.clone().borrow().get(id.as_str()) {
        value.clone()
      } else {
        Error(format!("Undefined variable: {}", id))
      }
    }
    List(l) => list::execute(*l, namespace_variables, local_variables),
    UnappliedList(l) => List(l),
    Vector(l) => Vector(
      l.into_iter()
        .map(|item| eval(item, namespace_variables.clone(), local_variables))
        .collect(),
    ),
    f @ Function(_) => f,
    n @ Number(_) => n,
    b @ Bool(_) => b,
    e @ Error(_) => e,
    s @ Str(_) => s,
    Nil => Nil,
  }
}
