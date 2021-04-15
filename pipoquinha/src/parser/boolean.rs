use pom::parser::*;

use crate::types::Boolean;
pub fn parser<'a>() -> Parser<'a, u8, Boolean> {
  let b_true = (seq(b"True") | seq(b"Real")).map(|_| Boolean::True);
  let b_false = (seq(b"False") | seq(b"Feiki")).map(|_| Boolean::False);

  (b_true | b_false).name("Boolean")
}

#[test]
fn parse_true() {
  let input = b"True";
  let output = parser().parse(input);

  assert_eq!(output, Ok(Boolean::True))
}

#[test]
fn parse_false() {
  let input = b"False";
  let output = parser().parse(input);

  assert_eq!(output, Ok(Boolean::False))
}
