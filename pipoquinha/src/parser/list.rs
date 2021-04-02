extern crate pom;

use pom::parser::*;

use crate::atom::*;

pub type List = Vec<Atom>;

pub fn list_parser<'a>() -> Parser<'a, u8, List> {
  sym(b'[') * space().opt() * list(atom(), space())
    - space().opt()
    - sym(b']')
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::atom::Atom::{Bool, Number};
  use crate::boolean::Boolean::*;

  #[test]
  fn parse_number_list_parser() {
    let input = b"[ 1 2 3 ]";
    let output = list_parser().parse(input);

    assert_eq!(Ok(vec![Number(1), Number(2), Number(3)]), output);
  }

  #[test]
  fn parse_multi_type_list_parser() {
    let input = b"[ 1 True 25 ]";
    let output = list_parser().parse(input);

    assert_eq!(Ok(vec![Number(1), Bool(True), Number(25)]), output)
  }

  #[test]
  fn parse_list_parsers_without_optional_spaces() {
    let input = b"[1 True]";
    let output = list_parser().parse(input);
    assert_eq!(Ok(vec![Number(1), Bool(True)]), output)
  }

  #[test]
  fn parse_empty_list_parser() {
    let input = b"[]";
    let output = list_parser().parse(input);
    assert_eq!(Ok(vec![]), output);
  }
}
