use pipoquinha::parser::{atom::Atom, boolean::Boolean};

use crate::{eval, NamespaceTable, VarTable};

pub fn not(arguments: Vec<Atom>, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.into_iter().next().unwrap(), namespace_variables, local_variables) {
      b @ Atom::Bool(_) => b.not(),
      e @ Atom::Error(_) => e,
      _ => Atom::Bool(Boolean::False),
    }
  } else {
    Atom::Error(format!(
      "Wrong number of arguments for 'not': expecting 1, found {}",
      arguments.len()
    ))
  }
}
