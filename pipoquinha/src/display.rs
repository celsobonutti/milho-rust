use std::fmt::{Display, Formatter, Result};

use crate::parser::atom::Function;
use crate::atom::Atom::{self, *};
use crate::boolean::Boolean;

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
      Identifier(i) => write!(f, "#{}", i),
      Function(fun) => {
          match *fun.clone() {
            Function {
                parameters,
                ..
            } => write!(f, "#{}", parameters.len())
        }
      }
      List(l) => {
        match l.clone().head {
            None => write!(f, "nil"),
            Some(h) => write!(f, "List with {} as head", h)
        }
      }
    }
  }
}
