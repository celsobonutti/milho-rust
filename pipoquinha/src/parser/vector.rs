extern crate pom;

use pom::parser::*;

use super::atom::{self, *};
use super::space::*;

pub type Vector = Vec<Atom>;

pub fn parser<'a>() -> Parser<'a, u8, Vector> {
  sym(b'[') * space().opt() * list(atom::parser(), space())
    - space().opt()
    - sym(b']')
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::atom::Atom::{Bool, self};
  use crate::parser::boolean::Boolean::*;
  use crate::types::number::Number;

  #[test]
  fn parse_number_vector() {
    let input = b"[ 1 2 3 ]";
    let output = parser().parse(input);

    assert_eq!(Ok(vec![Atom::Number(Number::new(1, 1).unwrap()), Atom::Number(Number::new(2, 1).unwrap()), Atom::Number(Number::new(3, 1).unwrap())]), output);
  }

  #[test]
  fn parse_multi_type_vector() {
    let input = b"[ 1 True 25 ]";
    let output = parser().parse(input);

    assert_eq!(Ok(vec![Atom::Number(Number::new(1, 1).unwrap()), Bool(True), Atom::Number(Number::new(25, 1).unwrap())]), output)
  }

  #[test]
  fn parse_vectors_without_optional_spaces() {
    let input = b"[1 True]";
    let output = parser().parse(input);
    assert_eq!(Ok(vec![Atom::Number(Number::new(1, 1).unwrap()), Bool(True)]), output)
  }

  #[test]
  fn parse_empty_vector() {
    let input = b"[]";
    let output = parser().parse(input);
    assert_eq!(Ok(vec![]), output);
  }
}
