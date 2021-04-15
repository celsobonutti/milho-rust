use std::fmt::{Display, Formatter, Result};

use crate::types::{
  Atom::{self, *},
  Boolean,
};

impl Display for Atom {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Number(x) => write!(f, "{}", x),
      Bool(Boolean::False) => write!(f, "False"),
      Bool(Boolean::True) => write!(f, "True"),
      Error(reason) => {
        write!(f, "Error: {}", reason)
      }
      Identifier(i) => write!(f, "{}", i),
      Function(fun) => write!(f, "fn#{}", fun.param_len()),
      Macro(m) => write!(f, "macro#{}", m.param_len()),
      MultiArityFn(_funs) => write!(f, "multi-arity-function"),
      List(l) => {
        let mut text = "(".to_string();

        for (index, element) in l.clone().into_iter().enumerate() {
          if index == 0 {
            text.push_str(&format!("{}", element));
          } else {
            text.push_str(&format!(" {}", element));
          }
        }

        text.push(')');
        write!(f, "{}", text)
      }
      Str(string) => write!(f, "\"{}\"", string),
      BuiltIn(built_in) => write!(f, "BuiltIn.{:?}", built_in),
      Nil => write!(f, "Nil"),
    }
  }
}
