extern crate pom;
use pom::parser::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Arithmetic {
  Add,
  Sub,
  Mul,
  Div,
}

pub fn arithmetic_ops<'a>() -> Parser<'a, u8, Arithmetic> {
  let operations = one_of(b"+-*/").repeat(1);

  operations.collect().map(|c| {
    use Arithmetic::*;
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
fn parse_arithmetic_ops() {
  use Arithmetic::*;
  let add = b"+";
  let sub = b"-";
  let mul = b"*";
  let div = b"/";

  assert_eq!(Ok(Add), arithmetic_ops().parse(add));
  assert_eq!(Ok(Sub), arithmetic_ops().parse(sub));
  assert_eq!(Ok(Mul), arithmetic_ops().parse(mul));
  assert_eq!(Ok(Div), arithmetic_ops().parse(div));
}
