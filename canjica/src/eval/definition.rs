use std::collections::HashMap;

use pipoquinha::parser::{
  atom::Atom::{self, *},
  identifier::is_builtin,
};

use crate::{eval, NamespaceTable, VarTable};

pub fn variable(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 2 {
    if let Identifier(name) = arguments.remove(0) {
      if !is_builtin(name.as_str()) {
        let namespace_vars = namespace_variables.clone();

        let value = eval(arguments.remove(0), namespace_variables, local_variables);

        let mut mutable_namespace = namespace_vars.borrow_mut();

        mutable_namespace.insert_module(name.clone(), value);

        drop(mutable_namespace);

        Identifier(name)
      } else {
        Error(format!(
          "Cannot define '{}' as a variable, as it's the name of a built-in",
          name
        ))
      }
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
  if arguments.len() == 3 {
    if let Identifier(name) = arguments.remove(0) {
      if !is_builtin(name.as_str()) {
        if let Vector(parameters) = arguments.remove(0) {
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
        } else {
          Error("Second argument of 'defn' must be a vector of identifiers.".to_string())
        }
      } else {
        Error(format!(
          "Cannot define '{}' as a function, as it's the name of a built-in",
          name
        ))
      }
    } else {
      Error("First argument of 'defn' must be a identifier.".to_string())
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'defn': was expecing 2, found {}",
      arguments.len()
    ))
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
          args.insert(name.clone(), value.clone());
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
        let result = eval(arguments.remove(0), namespace_variables, &args);

        result
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
