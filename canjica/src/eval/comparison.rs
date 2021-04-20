use pipoquinha::types::{Atom, Boolean};

use crate::{eval, NamespaceTable};

pub fn eq(arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  let mut arguments = arguments.into_iter();
  if let Some(head) = arguments.next() {
    let mut res = Atom::Bool(Boolean::True);
    match eval(head, namespace_variables.clone()) {
      e @ Atom::Error(_) => return e,
      base => {
        while let Some(argument) = arguments.next() {
          match eval(argument, namespace_variables.clone()) {
            error @ Atom::Error(_) => return error,
            value @ _ if value != base => res = Atom::Bool(Boolean::False),
            _ => (),
          }
        }
      }
    }

    res
  } else {
    Atom::Error("Wrong number of arguments for '=': was expecting at least 1, found 0".to_string())
  }
}

pub fn gt(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  match &arguments[..] {
    [] => Atom::Error(
      "Wrong number of arguments for '>': was expecting at least 1, found 0".to_string(),
    ),
    [..] => match eval(arguments.remove(0), namespace_variables.clone()) {
      e @ Atom::Error(_) => e,
      Atom::Number(mut n) => {
        for item in arguments {
          match eval(item, namespace_variables.clone()) {
            Atom::Number(m) => {
              if n < m {
                return Atom::Bool(Boolean::False);
              } else {
                n = m
              }
            }
            e @ Atom::Error(_) => return e,
            result => {
              return Atom::Error(format!(
                "Every argument in a comparison should be a number, but {} is a {}.",
                result,
                result.type_of()
              ))
            }
          }
        }
        Atom::Bool(Boolean::True)
      }
      result => Atom::Error(format!(
        "Every argument in a comparison should be a number, but {} is a {}.",
        result,
        result.type_of()
      )),
    },
  }
}
