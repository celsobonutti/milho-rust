use std::collections::HashMap;

use pipoquinha::parser::{
  atom::Atom::{self, *},
  identifier::is_builtin,
};

use crate::{eval, VarTable};

pub fn variable(mut arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 2 {
    if let Identifier(name) = arguments.remove(0) {
      if !is_builtin(name.as_str()) {
        let value = eval(arguments.remove(0), variables);

        VariableInsertion(name, Box::new(value))
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

pub fn function(mut arguments: Vec<Atom>, _variables: &VarTable) -> Atom {
  if arguments.len() == 3 {
    if let Identifier(name) = arguments.remove(0) {
      if !is_builtin(name.as_str()) {
        if let Vector(parameters) = arguments.remove(0) {
          let function = Atom::new_function(parameters, arguments.remove(0));

          if function.is_error() {
            function
          } else {
            VariableInsertion(name, Box::new(function))
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

pub fn anonymous_function(mut arguments: Vec<Atom>, _variables: &VarTable) -> Atom {
  if arguments.len() == 2 {
    if let Vector(parameters) = arguments.remove(0) {
      Atom::new_function(parameters, arguments.remove(0))
    } else {
      Error("Second argument of 'defn' must be a list of identifiers.".to_string())
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'fn': was expecing 2, found {}",
      arguments.len()
    ))
  }
}

pub fn local_variables(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  match arguments.as_slice() {
    [Vector(vars), atom] => {
      let mut pairs = vars.chunks_exact(2);

      let mut local_table: VarTable = HashMap::new();

      local_table.extend(variables.clone());

      while let Some([key, value]) = pairs.next() {
        if let Identifier(name) = key {
          local_table.insert(name.clone(), eval(value.clone(), &local_table));
        } else {
          return Error("Something is wrong. Looks like one of your variables is not using an identifier as its name.".to_string());
        }
      }

      if pairs.remainder().len() > 0 {
        return Error(
          "Something is wrong. Looks like we have an odd number of values in the key-value vector."
            .to_string(),
        );
      }

      eval(atom.clone(), &local_table)
    }
    _ => Error("Wrong number of arguments for let".to_string()),
  }
}
