use pipoquinha::types::Atom;

use crate::{eval, NamespaceTable};

pub fn concatenate(arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  let mut result = Vec::new();

  for item in arguments {
    if let Atom::Vector(v) = eval(item, namespace_variables.clone()) {
      result.extend(v);
    } else {
      return Atom::Error("Cannot concatenate non-vector value".to_string());
    }
  }

  Atom::Vector(result)
}
