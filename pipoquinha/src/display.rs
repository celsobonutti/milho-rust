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
        let mut text = String::from("[ ");
        for item in vector {
          text.push_str(&format!("{} ", *item));
        }
        text.push(']');
        write!(f, "{}", text)
      }
      Error(reason) => {
        write!(f, "Error: {}", reason)
      }
      Identifier(i) => write!(f, "{}", i),
      Function(fun) => write!(f, "fn#{}", fun.param_len()),
      List(l) => {
        let mut text = String::from("( ");

        let list = l.clone();

        if l.head.is_some() {
          text.push_str(&format!("{} ", list.head.unwrap()));

          for item in list.tail {
            text.push_str(&format!("{} ", item));
          }
        }

        text.push(')');
        write!(f, "{}", text)
      }
      UnappliedList(l) => write!(f, "Unapplied: {:?}", l),
      Str(string) => write!(f, "\"{}\"", string),
      Nil => write!(f, "Nil"),
    }
  }
}
