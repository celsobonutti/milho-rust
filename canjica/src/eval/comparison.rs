use pipoquinha::parser::list::List;
use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::boolean::Boolean;

use crate::{eval, VarTable};

pub fn eq(list: List, variables: &VarTable) -> Atom {
  let mut arguments = list.tail.into_iter();
  if let Some(head) = arguments.next() {
    let mut res = Bool(Boolean::True);
    let base = eval(head, variables);

    while let Some(argument) = arguments.next() {
      match eval(argument, variables) {
        error @ Error(_) => return error,
        value @ _ if value != base => res = Bool(Boolean::False),
        _ => (),
      }
    }
    res
  } else {
    Error("Not enough arguments for comparison".to_string())
  }
}
