extern crate pom;

use pom::parser::*;

use super::atom::*;
use super::space::*;

pub type Vector = Vec<Atom>;

pub fn parser<'a>() -> Parser<'a, u8, Vector> {
  sym(b'[') * space().opt() * list(atom(), space())
    - space().opt()
    - sym(b']')
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::atom::Atom::{Bool, Number};
  use crate::parser::boolean::Boolean::*;

  #[test]
  fn parse_number_vector() {
    let input = b"[ 1 2 3 ]";
    let output = parser().parse(input);

    assert_eq!(Ok(vec![Number(1), Number(2), Number(3)]), output);
  }

  #[test]
  fn parse_multi_type_vector() {
    let input = b"[ 1 True 25 ]";
    let output = parser().parse(input);

    assert_eq!(Ok(vec![Number(1), Bool(True), Number(25)]), output)
  }

  #[test]
  fn parse_vectors_without_optional_spaces() {
    let input = b"[1 True]";
    let output = parser().parse(input);
    assert_eq!(Ok(vec![Number(1), Bool(True)]), output)
  }

  #[test]
  fn parse_empty_vector() {
    let input = b"[]";
    let output = parser().parse(input);
    assert_eq!(Ok(vec![]), output);
  }
}
