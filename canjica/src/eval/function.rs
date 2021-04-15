use pipoquinha::types::{
  Atom::{self, *},
  Function, List,
};

use crate::{eval, NamespaceTable};

pub fn multi_arity_function(
  mut functions: Vec<Function>,
  arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
) -> Atom {
  if let Some(index) = functions
    .iter()
    .position(|function| function.param_len() == arguments.len() && !function.variadic)
  {
    execute(functions.remove(index), arguments, namespace_variables)
  } else if let Some(index) = functions
    .iter()
    .position(|function| function.variadic && arguments.len() >= function.param_len() - 1)
  {
    execute(functions.remove(index), arguments, namespace_variables)
  } else {
    Error("Wrong number of arguments for function".to_string())
  }
}

pub fn execute(
  mut function: Function,
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
) -> Atom {
  if function.param_len() == arguments.len() && !function.variadic {
    let mut added_vars = Vec::new();

    function
      .parameters
      .into_iter()
      .zip(
        arguments
          .into_iter()
          .map(|atom| eval(atom, namespace_variables.clone())),
      )
      .for_each(|(key, value)| {
        added_vars.push(key.clone());
        namespace_variables
          .borrow_mut()
          .insert_local_var(&key, value);
      });

    let result = eval(function.atom, namespace_variables.clone());

    added_vars
      .iter()
      .for_each(|v| namespace_variables.borrow_mut().drop_local_var(v));

    result
  } else if function.variadic {
    if arguments.len() >= function.param_len() - 1 {
      let mut added_vars = Vec::new();

      for _ in 1..function.param_len() {
        let key = function.parameters.remove(0);
        let value = eval(arguments.remove(0), namespace_variables.clone());

        added_vars.push(key.clone());

        namespace_variables
          .borrow_mut()
          .insert_local_var(&key, value);
      }

      let key = function.parameters.remove(0);

      added_vars.push(key.clone());

      let value = Atom::List(List::from_vec(
        arguments
          .into_iter()
          .map(|atom| eval(atom, namespace_variables.clone()))
          .collect(),
      ));

      let mut mutable_ref = namespace_variables.borrow_mut();

      mutable_ref.insert_local_var(&key, value);

      drop(mutable_ref);

      let result = eval(function.atom, namespace_variables.clone());

      added_vars
        .iter()
        .for_each(|v| namespace_variables.borrow_mut().drop_local_var(v));

      result
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

pub fn execute_macro(
  mut function: Function,
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
) -> Atom {
  if function.param_len() == arguments.len() && !function.variadic {
    let mut added_vars = Vec::new();

    function
      .parameters
      .into_iter()
      .zip(arguments.into_iter())
      .for_each(|(key, value)| {
        added_vars.push(key.clone());
        namespace_variables
          .clone()
          .borrow_mut()
          .insert_local_var(&key, value);
      });

    eval(function.atom, namespace_variables)
  } else if function.variadic {
    if arguments.len() >= function.param_len() - 1 {
      let mut added_vars = Vec::new();

      for _ in 1..function.param_len() {
        let key = function.parameters.remove(0);
        let value = arguments.remove(0);

        added_vars.push(key.clone());

        namespace_variables
          .borrow_mut()
          .insert_local_var(&key, value);
      }

      let key = function.parameters.remove(0);

      added_vars.push(key.clone());

      let value = Atom::List(List::from_vec(arguments));

      let mut mutable_ref = namespace_variables.borrow_mut();

      mutable_ref.insert_local_var(&key, value);

      drop(mutable_ref);

      let result = eval(function.atom, namespace_variables.clone());

      added_vars
        .iter()
        .for_each(|v| namespace_variables.borrow_mut().drop_local_var(v));

      result
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
