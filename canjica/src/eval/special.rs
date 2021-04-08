use pipoquinha::parser::atom::Atom;
use pipoquinha::parser::boolean::Boolean;

use crate::{eval, VarTable};

pub fn do_function(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
    let mut atom = Atom::Nil;
    for item in arguments {
        atom = eval(item, variables);
    }
    atom
}

pub fn if_fun(parameters: Vec<Atom>, variables: &VarTable) -> Atom {
  if parameters.len() == 3 {
    let condition = eval(parameters[0].clone(), variables);
    let result = if condition == Atom::Bool(Boolean::False) {
      parameters[2].clone()
    } else {
      parameters[1].clone()
    };

    eval(result, variables)
  } else {
    Atom::Error("Wrong number of parameters for if".to_string())
  }
}
