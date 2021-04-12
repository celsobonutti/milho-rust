use std::fmt::{Display, Formatter, Result};

use crate::parser::atom::Atom::{self, *};
use crate::parser::boolean::Boolean;

impl Display for Atom {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Number(x) => write!(f, "{}", x),
      Bool(Boolean::False) => write!(f, "False"),
      Bool(Boolean::True) => write!(f, "True"),
      Vector(vector) => {
        let mut text = String::from("[");
        for (index, item) in vector.into_iter().enumerate() {
          if index == 0 {
            text.push_str(&format!("{}", *item));
          } else {
            text.push_str(&format!(" {}", *item));
          }
        }
        text.push(']');
        write!(f, "{}", text)
      }
      Error(reason) => {
        write!(f, "Error: {}", reason)
      }
      Identifier(i) => write!(f, "{}", i),
      Function(fun) => write!(f, "fn#{}", fun.param_len()),
      Macro(m) => write!(f, "macro#{}", m.param_len()),
      MultiArityFn(_funs) => write!(f, "multi-arity-function"),
      List(l) => {
        let mut text = String::from("(");

        let list = l.clone();

        if l.head.is_some() {
          text.push_str(&format!("{} ", list.head.unwrap()));

          for (index, item) in list.tail.into_iter().enumerate() {
            if index == 0 {
              text.push_str(&format!("{}", item));
            } else {
              text.push_str(&format!(" {}", item));
            }
          }
        }

        text.push(')');
        write!(f, "{}", text)
      }
      Str(string) => write!(f, "\"{}\"", string),
      BuiltIn(string) => write!(f, "built-in{}", string),
      Nil => write!(f, "Nil"),
    }
  }
}
