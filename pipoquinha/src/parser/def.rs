use pom::parser::*;

use crate::{
  atom::{atom, space, Atom},
  boolean::Boolean,
  identifier::identifier,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
  pub name: String,
  pub value: Atom,
}

pub fn def<'a>() -> Parser<'a, u8, Variable> {
  let rules = sym(b'(') * space().opt() * seq(b"def") * space() * identifier() - space() + atom()
    - space().opt()
    - sym(b')');

  rules
    .map(|(name, value)| Variable { name, value })
    .name("Variable definition")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn number_var() {
    let input = b"(def x 5)";
    let expected = Variable {
      name: String::from("x"),
      value: Atom::Number(5),
    };

    assert_eq!(Ok(expected), def().parse(input));
  }

  
  #[test]
  fn list_var() {
    let input = b"(def my-list [6 8 True])";
    let expected = Variable {
      name: String::from("my-list"),
      value: Atom::List(vec![Atom::Number(6), Atom::Number(8), Atom::Bool(Boolean::True)])
    };

    assert_eq!(Ok(expected), def().parse(input));
  }
}
