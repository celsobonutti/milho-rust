use std::collections::HashMap;

use pipoquinha::parser::{
  atom::Atom::{self, *},
  list::List,
};

use crate::{eval, NamespaceTable, VarTable};

pub fn variable(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 2 {
    if let Identifier(name) = arguments.remove(0) {
      let namespace_vars = namespace_variables.clone();

      let value = eval(arguments.remove(0), namespace_variables, local_variables);

      let mut mutable_namespace = namespace_vars.borrow_mut();

      mutable_namespace.insert_module(name.clone(), value);

      drop(mutable_namespace);

      Identifier(name)
    } else {
      Error("First argument of 'def' must be a identifier.".to_string())
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'def': was expecing 2, found {}",
      arguments.len()
    ))
  }
}

pub fn function(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  _local_variables: &VarTable,
) -> Atom {
  match arguments.as_slice() {
    [Identifier(_name), Vector(parameters), _]
      if parameters.iter().all(|atom| atom.is_identifier()) =>
    {
      let name = arguments.remove(0).unwrap_id();
      let parameters = arguments.remove(0).unwrap_vector();

      let function = Atom::new_function(parameters, arguments.remove(0));

      if function.is_error() {
        function
      } else {
        let namespace_vars = namespace_variables.clone();

        let mut mutable_namespace = namespace_vars.borrow_mut();

        mutable_namespace.insert_module(name.clone(), function);

        drop(mutable_namespace);

        Identifier(name)
      }
    }
    [Identifier(name), Vector(_parameters), _] => Error(format!(
      "Cannot define '{}' as a function, as it's the name of a built-in",
      name
    )),
    [Identifier(_name), bodies @ ..]
      if bodies.len() > 0 && bodies.iter().all(|l| l.is_list()) =>
    {
      let name = arguments.remove(0).unwrap_id();
      let bodies = arguments;
      let mut has_variadic_shown_up = false;
      let result =
        bodies
          .into_iter()
          .fold(MultiArityFn(Box::new(vec![])), |acc, body| match acc {
            e @ Error(_) => e,
            MultiArityFn(mut v) => match body {
              List(l) => match *l {
                List {
                  head: Some(Vector(items)),
                  mut tail,
                } if items.iter().all(|item| item.is_identifier()) && tail.len() == 1 => {
                  match Atom::new_function(items, tail.remove(0)) {
                    e @ Error(_) => e,
                    Function(new_function) => {
                      if v.iter().any(|f| f.param_len() == new_function.param_len() && f.variadic == new_function.variadic) {
                        return Error("Can't have two bodies with the same arity.".to_string())
                      } else if has_variadic_shown_up && new_function.variadic {
                        return Error("Can't have two variadic bodies for the same function.".to_string());
                      } else {
                        if new_function.variadic { 
                          has_variadic_shown_up = true;
                        }

                       v.push(*new_function);
                       MultiArityFn(v)  
                      }
                    }
                    x => x,
                  }
                }
                _ => Error("Wrong type of argument for multi-arity functions: all bodies need to be a list composed of a vector with identifiers followed by only one expression".to_string())
              },
              x => x,
            },
            x => x,
          });

      match result {
        e@Error(_) => e,
        fns@MultiArityFn(_) => {
            let namespace_vars = namespace_variables.clone();
            
            let mut mutable_namespace = namespace_vars.borrow_mut();

            mutable_namespace.insert_module(name.clone(), fns);

            drop(mutable_namespace);

            Identifier(name)
        },
        buggy => Error(format!("Something is wrong: was expecting the value to be a MultiArityFunction, but it is: {}", buggy))
      }
    }
    _ => Error(format!(
      "Wrong number of arguments for 'defn': was expecing 3, found {}",
      arguments.len()
    )),
  }
}

pub fn anonymous_function(
  mut arguments: Vec<Atom>,
  _namespace_variables: NamespaceTable,
  _local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 2 {
    if let Vector(parameters) = arguments.remove(0) {
      Atom::new_function(parameters, arguments.remove(0))
    } else {
      Error("Second argument of 'fn' must be a list of identifiers.".to_string())
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'fn': was expecing 2, found {}",
      arguments.len()
    ))
  }
}

pub fn local_variables(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 2 {
    if let Vector(vars) = arguments.remove(0) {
      let mut pairs = vars.chunks_exact(2).into_iter();

      let mut args = HashMap::new();

      args.extend(local_variables.clone());

      while let Some([key, value]) = pairs.next() {
        if let Identifier(name) = key {
          args.insert(
            name.clone(),
            eval(value.clone(), namespace_variables.clone(), &args),
          );
        } else {
          return Error("Something is wrong. Looks like one of your variables is not using an identifier as its name.".to_string());
        }
      }

      if pairs.remainder().len() > 0 {
        return Error(
          "Something is wrong. Looks like we have an odd number of values in the key-value vector."
            .to_string(),
        );
      } else {
        eval(arguments.remove(0), namespace_variables, &args)
      }
    } else {
      return Error(
        "Wrong argument type for let: its first argument needs to be a vector of key-value pairs"
          .to_string(),
      );
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'let': expecting 2, found {}",
      arguments.len()
    ))
  }
}
