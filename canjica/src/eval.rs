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

use pipoquinha::parser::atom::Atom::{self, *};

use super::{NamespaceTable, VarTable};

pub fn eval(atom: Atom, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  match atom {
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
    Vector(l) => Vector(
      l.into_iter()
        .map(|item| eval(item, namespace_variables.clone(), local_variables))
        .collect(),
    ),
    Number(x) => Number(x.simplify()),
    f @ Function(_) => f,
    m @ Macro(_) => m,
    maf @ MultiArityFn(_) => maf,
    b @ Bool(_) => b,
    e @ Error(_) => e,
    s @ Str(_) => s,
    b @ BuiltIn(_) => b,
    Nil => Nil,
  }
}
