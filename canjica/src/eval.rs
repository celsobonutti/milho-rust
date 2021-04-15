mod arithmetic;
mod boolean;
mod comparison;
mod definition;
mod function;
mod io;
mod list;
mod special;
mod string;

use pipoquinha::types::Atom::{self, *};

use super::NamespaceTable;

pub fn eval(atom: Atom, namespace_variables: NamespaceTable) -> Atom {
  match atom {
    Identifier(id) => {
      if let Some(value) = namespace_variables.clone().borrow().get(&id) {
        value.clone()
      } else {
        Error(format!("Undefined variable: {}", id))
      }
    }
    List(l) => list::execute(l, namespace_variables),
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
