use pom::parser::*;

use super::atom::{atom, Atom};
use super::space::space;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct List {
  pub head: Option<Atom>,
  pub tail: Vec<Atom>,
}

pub fn list_parser<'a>() -> Parser<'a, u8, List> {
  let parser = sym(b'(') * space().opt() * atom().opt() - space() + list(atom(), space())
    - space().opt()
    - sym(b')');

  parser.map(|(head, tail)| List { head, tail }).name("List")
}

#[cfg(test)]
mod tests {
use super::{List, list_parser};
use crate::atom::Atom::*;

  #[test]
  fn parse_sum_list_parser() {
    let input = b"(+ 3 3 4)";
    let output = list_parser().parse(input);

    assert_eq!(
      output,
      Ok(List {
        head: Some(Identifier(String::from("+"))),
        tail: vec![Number(3), Number(3), Number(4)]
      })
    );
  }

  #[test]
  fn parse_sum_within_sum() {

    let input = b"(+ 3 (+ 5 3))";
    let output = list_parser().parse(input);
    let internal_sum = Box::new(list_parser().parse(b"(+ 5 3)").unwrap());

    assert_eq!(
      Ok(List {
        head: Some(Identifier(String::from("+"))),
        tail: vec![Number(3), List(internal_sum)]
      }),
      output
    )
  }
}
