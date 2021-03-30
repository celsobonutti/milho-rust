use pom::parser::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Boolean {
  True,
  False
}

pub fn boolean<'a>() -> Parser<'a, u8, Boolean> {
  let b_true = seq(b"True").map(|_| Boolean::True);
  let b_false = seq(b"False").map(|_| Boolean::False);
  
  (b_true | b_false).name("Boolean")
}

pub fn negate(b: Boolean) -> Boolean {
  match b {
    Boolean::True => Boolean::False,
    Boolean::False => Boolean::True
  }
}

#[test]
fn parse_true() {
    let input = b"True";
    let output = boolean().parse(input);

    assert_eq!(output, Ok(Boolean::True))
}

#[test]
fn parse_false() {
    let input = b"False";
    let output = boolean().parse(input);

    assert_eq!(output, Ok(Boolean::False))
}
