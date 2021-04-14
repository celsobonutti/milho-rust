use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::types::number::Number;

use crate::{eval, NamespaceTable, VarTable};

pub fn add(
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, namespace_variables.clone(), local_variables))
    .fold(Atom::Number(Number::new(0, 1).unwrap()), |acc, val| {
      acc.add(&val)
    })
}

pub fn negate(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  match arguments.as_slice() {
    [_] => eval(arguments.remove(0), namespace_variables, local_variables).negate(),
    _ => Error(format!(
      "Wrong number of arguments for 'negate': was expecing 1, found {}",
      arguments.len()
    )),
  }
}

pub fn invert(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  match arguments.as_slice() {
    [_] => eval(arguments.remove(0), namespace_variables, local_variables).invert(),
    _ => Error(format!(
      "Wrong number of arguments for 'negate': was expecing 1, found {}",
      arguments.len()
    )),
  }
}

pub fn multiply(
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, namespace_variables.clone(), local_variables))
    .fold(Atom::Number(Number::new(1, 1).unwrap()), |acc, val| {
      acc.mul(&val)
    })
}
