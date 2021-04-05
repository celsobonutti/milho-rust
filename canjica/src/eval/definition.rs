use std::collections::HashMap;

use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::list::List;

use crate::{eval, VarTable};

pub fn variable(list: List, variables: &VarTable) -> Atom {
  match list.tail.as_slice() {
    [Identifier(name), value] => {
      let value = eval(value.clone(), variables);

      VariableInsertion(name.clone(), Box::new(value))
    }
    _ => Error("Wrong number of arguments for def".to_string()),
  }
}

pub fn function(list: List, _variables: &VarTable) -> Atom {
  match list.tail.as_slice() {
    [Identifier(name), Vector(parameters), atom] => {
      let function = Atom::new_function(parameters, atom);

      VariableInsertion(name.clone(), Box::new(function))
    }
    _ => Error("Wrong number of arguments for defn".to_string()),
  }
}

pub fn anonymous_function(list: List, _variables: &VarTable) -> Atom {
  match list.tail.as_slice() {
    [Vector(parameters), atom] => Atom::new_function(parameters, atom),
    _ => Error("Wrong type of arguments for fn".to_string()),
  }
}

pub fn local_variables(list: List, variables: &VarTable) -> Atom {
  match list.tail.as_slice() {
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
