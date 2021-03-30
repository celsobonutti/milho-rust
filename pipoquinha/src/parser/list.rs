extern crate pom;

use pom::parser::*;

use super::atom::{
  Atom::{Bool, Number},
  *,
};
use super::boolean::Boolean::*;

pub type List = Vec<Atom>;

pub fn list<'a>() -> Parser<'a, u8, List> {
  let spaced_atom = space() * atom();
  let expression = sym(b'[') * spaced_atom.repeat(0..) - space().opt() - sym(b']');

  expression
}

#[test]
fn parse_number_list() {
  let input = b"[ 1 2 3 ]";
  let output = list().parse(input);

  assert_eq!(Ok(vec![Number(1), Number(2), Number(3)]), output);
}

#[test]
fn parser_multi_type_list() {
  let input = b"[ 1 True 25 ]";
  let output = list().parse(input);

  assert_eq!(Ok(vec![Number(1), Bool(True), Number(25)]), output)
}
