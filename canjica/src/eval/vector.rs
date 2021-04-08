use pipoquinha::parser::atom::Atom;

use crate::{eval, VarTable};

pub fn head(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.into_iter().next().unwrap(), variables) {
      Atom::Vector(v) => v.into_iter().next().unwrap_or(Atom::Error(
        "Cannot get the head of an empty list".to_string(),
      )),
      e @ Atom::Error(_) => e,
      _ => Atom::Error("Type Error: you can only get the head of a vector".to_string()),
    }
  } else {
    Atom::Error(format!(
      "Wrong number of arguments for 'head': expecting 1, found {}",
      arguments.len()
    ))
  }
}

pub fn tail(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.into_iter().next().unwrap(), variables) {
      Atom::Vector(v) => {
        let mut iter = v.into_iter();
        iter.next();

        let remainder = iter.collect();

        Atom::Vector(remainder)
      }
      e @ Atom::Error(_) => e,
      _ => Atom::Error("Type Error: you can only get the head of a vector".to_string()),
    }
  } else {
    Atom::Error(format!(
      "Wrong number of arguments for 'head': expecting 1, found {}",
      arguments.len()
    ))
  }
}

pub fn concatenate(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  let mut result = Vec::new();

  for item in arguments {
    if let Atom::Vector(v) = eval(item, variables) {
        result.extend(v);
    } else {
        return Atom::Error("Cannot concatenate non-vector value".to_string());
    }
  }

  Atom::Vector(result)
}
