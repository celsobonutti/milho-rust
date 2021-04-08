use pipoquinha::parser::atom::Atom;
use pipoquinha::parser::boolean::Boolean;

use crate::{eval, NamespaceTable, VarTable};

use super::list::execute;

pub fn do_function(arguments: Vec<Atom>, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  let mut atom = Atom::Nil;
  for item in arguments {
    atom = eval(item, namespace_variables.clone(), local_variables);
  }
  atom
}

pub fn if_fun(mut parameters: Vec<Atom>, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  if parameters.len() == 3 {
    let condition = eval(parameters.remove(0), namespace_variables.clone(), local_variables);
    let result = if condition == Atom::Bool(Boolean::False) {
      parameters.remove(1)
    } else {
      parameters.remove(0)
    };

    eval(result, namespace_variables, local_variables)
  } else {
    Atom::Error("Wrong number of parameters for if".to_string())
  }
}

#[allow(unreachable_code)]
pub fn loop_function(mut parameters: Vec<Atom>, namespace_variables: NamespaceTable, local_variables: &VarTable) -> Atom {
  if parameters.len() == 1 {
    if let Atom::List(l) = parameters.remove(0) {
      loop {
        execute(*l.clone(), namespace_variables.clone(), local_variables);
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
