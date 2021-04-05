use std::collections::HashMap;

use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::list::List;

use crate::{eval, VarTable};

pub fn execute(list: List, variables: &VarTable) -> Atom {
  if let Some(Identifier(name)) = &list.head {
    if let Some(Function(function)) = variables.get(name) {
      if function.param_len() == list.tail.len() {
        let function = function.clone();

        let mut local_table = HashMap::new();

        function
          .parameters
          .into_iter()
          .zip(list.tail.into_iter())
          .for_each(|(key, value)| {
            local_table.insert(key, eval(value, variables));
          });

        local_table.extend(variables.clone());

        eval(function.atom, &mut local_table)
      } else {
        Error(format!(
          "Wrong number of arguments for {}: expected {}, got {})",
          name,
          function.param_len(),
          list.tail.len()
        ))
      }
    } else if let Some(l@List(_)) = variables.get(name) {
        eval(l.clone(), variables)
    } else if let Some(_) = variables.get(name) {
      Error(format!("Cannot invoke {}, as it's not a function", name))
    } else {
      Error(format!("Undefined function: {}", name))
    }
  } else {
    Error("Cannot invoke this, as it's not a function".to_string())
  }
}
