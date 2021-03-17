extern crate pom;
use pom::parser::*;
use pom::Parser;

#[derive(Debug, Eq, PartialEq)]
pub enum BuiltIn {
  Add,
  Sub,
  Mul,
  Div,
}

pub fn integer_built_ins() -> Parser<u8, BuiltIn> {
  let built_ins = one_of(b"+-*/").repeat(1);

  built_ins.collect().map(|c| {
    use BuiltIn::*;
    match (c[0]) as char {
      '+' => Add,
      '-' => Sub,
      '*' => Mul,
      '/' => Div,
      _ => panic!("Something is very, very wrong with this parser."),
    }
  })
}

#[test]
fn parse_integer_built_ins() {
  use BuiltIn::*;
  let add = b"+";
  let sub = b"-";
  let mul = b"*";
  let div = b"/";

  assert_eq!(Ok(Add), integer_built_ins().parse(add));
  assert_eq!(Ok(Sub), integer_built_ins().parse(sub));
  assert_eq!(Ok(Mul), integer_built_ins().parse(mul));
  assert_eq!(Ok(Div), integer_built_ins().parse(div));
}
