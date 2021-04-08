use crate::{eval, VarTable};
use pipoquinha::parser::{
  atom::Atom::{self, *},
  boolean::Boolean,
};

pub fn if_fun(parameters: Vec<Atom>, variables: &VarTable) -> Atom {
  if parameters.len() == 3 {
    let condition = eval(parameters[0].clone(), variables);
    let result = if condition == Bool(Boolean::False) {
      parameters[2].clone()
    } else {
      parameters[1].clone()
    };

    eval(result, variables)
  } else {
    Error("Wrong number of parameters for if".to_string())
  }
}
