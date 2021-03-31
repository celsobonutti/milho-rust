extern crate pom;

use pom::parser::*;

use crate::atom::*;

pub type List = Vec<Atom>;

pub fn list<'a>() -> Parser<'a, u8, List> {
  let spaced_atom = space() * atom();
  let expression = sym(b'[') * (space().opt() * atom()).opt() + spaced_atom.repeat(0..)
    - space().opt()
    - sym(b']');

  expression.name("List").map(|(head, mut rest)| match head {
    None => vec![],
    Some(h) => {
      rest.insert(0, h);
      rest
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::atom::Atom::{Bool, Number};
  use crate::boolean::Boolean::*;

  #[test]
  fn parse_number_list() {
    let input = b"[ 1 2 3 ]";
    let output = list().parse(input);

    assert_eq!(Ok(vec![Number(1), Number(2), Number(3)]), output);
  }

  #[test]
  fn parse_multi_type_list() {
    let input = b"[ 1 True 25 ]";
    let output = list().parse(input);

    assert_eq!(Ok(vec![Number(1), Bool(True), Number(25)]), output)
  }

  #[test]
  fn parse_lists_without_optional_spaces() {
    let input = b"[1 True]";
    let output = list().parse(input);
    assert_eq!(Ok(vec![Number(1), Bool(True)]), output)
  }

  #[test]
  fn parse_empty_list() {
    let input = b"[]";
    let output = list().parse(input);
    assert_eq!(Ok(vec![]), output);
  }
}
