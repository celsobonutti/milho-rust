use pipoquinha::parser::{
  atom::Atom::{self, *},
  list::List,
};

use crate::{eval, NamespaceTable};

pub fn variable(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 2 {
    if let Identifier(name) = arguments.remove(0) {
      let namespace_vars = namespace_variables.clone();

      let value = eval(arguments.remove(0), namespace_variables);

      let mut mutable_namespace = namespace_vars.borrow_mut();

      mutable_namespace.insert_global_var(&name, value);

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

fn multi_arity_fn(acc: Atom, body: Atom, has_variadic_shown_up: &mut bool) -> Atom {
  let invalid_body_error = Error("Wrong type of argument for multi-arity function: all bodies need to be a list composed of a vector with identifiers followed by only one expression".to_string());
  if let MultiArityFn(mut v) = acc {
    if let List(l) = body {
      if let List {
        head: Some(Vector(items)),
        mut tail,
      } = *l
      {
        if items.iter().all(|item| item.is_identifier()) && tail.len() == 1 {
          return match Atom::new_function(items, tail.remove(0), false) {
            e @ Error(_) => return e,

            Function(new_function) => {
              if v.iter().any(|f| {
                f.param_len() == new_function.param_len() && f.variadic == new_function.variadic
              }) {
                Error("Can't have two bodies with the same arity.".to_string())
              } else if *has_variadic_shown_up && new_function.variadic {
                return Error("Can't have two variadic bodies for the same function.".to_string());
              } else {
                if new_function.variadic {
                  *has_variadic_shown_up = true;
                }

                v.push(*new_function);
                MultiArityFn(v)
              }
            }

            _ => invalid_body_error,
          };
        };
      };
    }
  }
  invalid_body_error
}

pub fn function(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  match arguments.as_slice() {
    [Identifier(_name), Vector(parameters), _]
      if parameters.iter().all(|atom| atom.is_identifier()) =>
    {
      let name = arguments.remove(0).unwrap_id();
      let parameters = arguments.remove(0).unwrap_vector();

      let function = Atom::new_function(parameters, arguments.remove(0), false);

      if function.is_error() {
        function
      } else {
        let namespace_vars = namespace_variables.clone();

        let mut mutable_namespace = namespace_vars.borrow_mut();

        mutable_namespace.insert_global_var(&name, function);

        drop(mutable_namespace);

        Identifier(name)
      }
    }
    [Identifier(_name), bodies @ ..] if bodies.len() > 0 && bodies.iter().all(|l| l.is_list()) => {
      let name = arguments.remove(0).unwrap_id();
      let bodies = arguments;
      let mut has_variadic_shown_up = false;
      let result = bodies
        .into_iter()
        .fold(MultiArityFn(Box::new(vec![])), |acc, body| {
          multi_arity_fn(acc, body, &mut has_variadic_shown_up)
        });

      match result {
        e @ Error(_) => e,
        fns @ MultiArityFn(_) => {
          let namespace_vars = namespace_variables.clone();

          let mut mutable_namespace = namespace_vars.borrow_mut();

          mutable_namespace.insert_global_var(&name, fns);

          drop(mutable_namespace);

          Identifier(name)
        }
        buggy => Error(format!(
          "Something is wrong: was expecting the value to be a MultiArityFunction, but it is: {}",
          buggy
        )),
      }
    }
    _ => Error(format!(
      "Wrong number of arguments for 'defn': was expecing 3, found {}",
      arguments.len()
    )),
  }
}

pub fn anonymous_function(mut arguments: Vec<Atom>, _namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 2 {
    if let Vector(parameters) = arguments.remove(0) {
      Atom::new_function(parameters, arguments.remove(0), false)
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

pub fn local_variables(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 2 {
    if let Vector(vars) = arguments.remove(0) {
      let mut pairs = vars.chunks_exact(2).into_iter();

      let mut inserted_vars = Vec::new();

      while let Some([key, value]) = pairs.next() {
        if let Identifier(name) = key {
          inserted_vars.push(name);

          namespace_variables
            .borrow_mut()
            .insert_local_var(name, eval(value.clone(), namespace_variables.clone()));
        } else {
          inserted_vars
            .iter()
            .for_each(|var| namespace_variables.borrow_mut().drop_local_var(var));
          return Error("Something is wrong. Looks like one of your variables is not using an identifier as its name.".to_string());
        }
      }

      if pairs.remainder().len() > 0 {
        inserted_vars
          .iter()
          .for_each(|var| namespace_variables.borrow_mut().drop_local_var(var));
        return Error(
          "Something is wrong. Looks like we have an odd number of values in the key-value vector."
            .to_string(),
        );
      } else {
        let value = eval(arguments.remove(0), namespace_variables.clone());

        inserted_vars
          .iter()
          .for_each(|var| namespace_variables.borrow_mut().drop_local_var(var));

        value
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

pub fn macro_d(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  match arguments.as_slice() {
    [Identifier(_name), Vector(parameters), _]
      if parameters.iter().all(|atom| atom.is_identifier()) =>
    {
      let name = arguments.remove(0).unwrap_id();
      let parameters = arguments.remove(0).unwrap_vector();

      let function = Atom::new_function(parameters, arguments.remove(0), true);

      if function.is_error() {
        function
      } else {
        let namespace_vars = namespace_variables.clone();

        let mut mutable_namespace = namespace_vars.borrow_mut();

        mutable_namespace.insert_global_var(&name, function);

        drop(mutable_namespace);

        Identifier(name)
      }
    }
    _ => Error(format!(
      "Wrong number of arguments for 'defmacro': was expecing 3, found {}",
      arguments.len()
    )),
  }
}
