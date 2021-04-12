use std::collections::HashMap;

use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::atom::Function;
use pipoquinha::parser::list::List;

use crate::{eval, NamespaceTable, VarTable};

pub fn multi_arity_function(
  mut functions: Vec<Function>,
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
    if let Some(index) = functions.iter().position(|function| function.param_len() == arguments.len() && !function.variadic) {
       execute(functions.remove(index), arguments, namespace_variables, local_variables)
    } else if let Some(index) = functions.iter().position(|function| function.variadic && arguments.len() >= function.param_len() - 1) { 
       execute(functions.remove(index), arguments, namespace_variables, local_variables)
    } else {
       Error("Wrong number of arguments for function".to_string())
    }
}

pub fn execute(
  mut function: Function,
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if function.param_len() == arguments.len() && !function.variadic {
    let mut new_local = HashMap::new();

    new_local.extend(local_variables.clone());

    function
      .parameters
      .into_iter()
      .zip(
        arguments
          .into_iter()
          .map(|atom| eval(atom, namespace_variables.clone(), local_variables)),
      )
      .for_each(|(key, value)| {
        new_local.insert(key, value);
      });

    eval(function.atom, namespace_variables, &new_local)
  } else if function.variadic {
    if arguments.len() >= function.param_len() - 1 {
      let mut new_local = HashMap::new();

      new_local.extend(local_variables.clone());

      for _ in 1..function.param_len() {
        new_local.insert(
          function.parameters.remove(0),
          eval(
            arguments.remove(0),
            namespace_variables.clone(),
            local_variables,
          ),
        );
      }

      new_local.insert(
        function.parameters.remove(0),
        Atom::List(Box::new(List::from_vec(
          arguments
            .into_iter()
            .map(|atom| eval(atom, namespace_variables.clone(), local_variables))
            .collect(),
        ))),
      );

      eval(function.atom, namespace_variables, &new_local)
    } else {
      Error(format!(
        "Wrong number of arguments for function: expected at least {}, got {}",
        function.param_len() - 1,
        arguments.len()
      ))
    }
  } else {
    Error(format!(
      "Wrong number of arguments for function: expected {}, got {})",
      function.param_len(),
      arguments.len()
    ))
  }
}
