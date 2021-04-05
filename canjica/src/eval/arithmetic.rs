use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::list::List;

use crate::eval;
use crate::VarTable;

pub fn add(list: List, variables: &VarTable) -> Atom {
  list
    .tail
    .into_iter()
    .map(|val| eval(val, variables))
    .fold(Number(0), |acc, val| acc.add(&val))
}

pub fn subtract(list: List, variables: &VarTable) -> Atom {
  if list.tail.len() == 1 {
    list.tail.first().unwrap().negate()
  } else {
    list
      .tail
      .into_iter()
      .map(|val| eval(val, variables))
      .reduce(|acc, val| acc.add(&val.negate()))
      .unwrap_or(Error("Not enough arguments for subtraction".to_string()))
  }
}

pub fn multiply(list: List, variables: &VarTable) -> Atom {
  list
    .tail
    .into_iter()
    .map(|val| eval(val, variables))
    .fold(Number(1), |acc, val| acc.mul(&val))
}

pub fn divide(list: List, variables: &VarTable) -> Atom {
  list
    .tail
    .into_iter()
    .map(|val| eval(val, variables))
    .reduce(|acc, val| acc.div(&val))
    .unwrap_or(Error("Not enough arguments for division".to_string()))
}
