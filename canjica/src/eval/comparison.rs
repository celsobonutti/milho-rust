use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::boolean::Boolean;

use crate::{eval, NamespaceTable, VarTable};

pub fn eq(arguments: Vec<Atom>, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  let mut arguments = arguments.into_iter();
  if let Some(head) = arguments.next() {
    let mut res = Bool(Boolean::True);
    let base = eval(head, namespace_variables.clone(), local_variables);
    while let Some(argument) = arguments.next() {
      match eval(argument, namespace_variables.clone(), local_variables) {
        error @ Error(_) => return error,
        value @ _ if value != base => res = Bool(Boolean::False),
        _ => (),
      }
    }
    res
  } else {
    Error("Wrong number of arguments for '=': was expecting at least 1, found 0".to_string())
  }
}
