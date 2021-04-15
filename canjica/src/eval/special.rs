use pipoquinha::types::{Atom, Boolean};

use crate::{eval, NamespaceTable};

use super::list::execute;

pub fn do_function(arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  let mut atom = Atom::Nil;
  for item in arguments {
    atom = eval(item, namespace_variables.clone());
  }
  atom
}

pub fn if_fun(mut parameters: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if parameters.len() == 3 {
    let condition = eval(parameters.remove(0), namespace_variables.clone());
    let result = if condition == Atom::Bool(Boolean::False) {
      parameters.remove(1)
    } else {
      parameters.remove(0)
    };

    eval(result, namespace_variables)
  } else {
    Atom::Error("Wrong number of parameters for if".to_string())
  }
}

#[allow(unreachable_code)]
pub fn loop_function(mut parameters: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if parameters.len() == 1 {
    if let Atom::List(l) = parameters.remove(0) {
      loop {
        execute(l.clone(), namespace_variables.clone());
      }
      Atom::Nil
    } else {
      Atom::Error("Cannot loop over non-list value".to_string())
    }
  } else {
    Atom::Error(format!(
      "Wrong number of parameters for 'loop': was expecting 1, found {}",
      parameters.len()
    ))
  }
}
