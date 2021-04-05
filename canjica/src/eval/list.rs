use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::list::List;

use super::{arithmetic, comparison, definition, function};
use crate::VarTable;

pub fn execute(list: List, variables: &VarTable) -> Atom {
  let fun_name = &list.head;
  match fun_name {
    Some(Identifier(x)) => match x.as_str() {
      "+" => arithmetic::add(list, variables),
      "-" => arithmetic::subtract(list, variables),
      "*" => arithmetic::multiply(list, variables),
      "/" => arithmetic::divide(list, variables),
      "=" => comparison::eq(list, variables),
      "cons" => cons(list, variables),
      "def" => definition::variable(list, variables),
      "defn" => definition::function(list, variables),
      "fn" => definition::anonymous_function(list, variables),
      "let" => definition::local_variables(list, variables),
      _ => function::execute(list, variables),
    },
    None => Error(String::from("Nil")),
    _ => Error(String::from("Not implemented")),
  }
}

fn cons(list: List, _variables: &VarTable) -> Atom {
    match &list.tail.as_slice() {
        [x, List(l)] => {
           List(Box::new(l.append(x.clone())))
        },
        _ => Error("Fodase".to_string())
    }
}
