use pipoquinha::parser::atom::Atom::{self, *};

use crate::{eval, NamespaceTable, VarTable};

pub fn add(
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, namespace_variables.clone(), local_variables))
    .fold(Number(0), |acc, val| acc.add(&val))
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

pub fn subtract(
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 1 {
    arguments.first().unwrap().negate()
  } else {
    arguments
      .into_iter()
      .map(|val| eval(val, namespace_variables.clone(), local_variables))
      .reduce(|acc, val| acc.add(&val.negate()))
      .unwrap_or(Error(
        "Wrong number of arguments for '-': was expecting at least 1, found 0".to_string(),
      ))
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
    .fold(Number(1), |acc, val| acc.mul(&val))
}

pub fn divide(
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, namespace_variables.clone(), local_variables))
    .reduce(|acc, val| acc.div(&val))
    .unwrap_or(Error(
      "Wrong number of arguments for '/': was expecting at least 1, found 0".to_string(),
    ))
}
