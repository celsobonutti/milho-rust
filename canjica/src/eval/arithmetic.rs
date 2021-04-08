use pipoquinha::parser::atom::Atom::{self, *};

use crate::eval;
use crate::VarTable;

pub fn add(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, variables))
    .fold(Number(0), |acc, val| acc.add(&val))
}

pub fn subtract(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    arguments.first().unwrap().negate()
  } else {
    arguments
      .into_iter()
      .map(|val| eval(val, variables))
      .reduce(|acc, val| acc.add(&val.negate()))
      .unwrap_or(Error(
        "Wrong number of arguments for '-': was expecting at least 1, found 0".to_string(),
      ))
  }
}

pub fn multiply(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, variables))
    .fold(Number(1), |acc, val| acc.mul(&val))
}

pub fn divide(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  arguments
    .into_iter()
    .map(|val| eval(val, variables))
    .reduce(|acc, val| acc.div(&val))
    .unwrap_or(Error(
      "Wrong number of arguments for '/': was expecting at least 1, found 0".to_string(),
    ))
}
