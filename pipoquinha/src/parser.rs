extern crate pom;
mod function;
mod integer;

pub use function::*;
pub use integer::*;
use pom::parser::*;
use pom::Parser;

#[derive(Debug, Eq, PartialEq)]
pub struct Expression {
  function: BuiltIn,
  values: Vec<i64>,
}

pub fn expression() -> Parser<u8, Expression> {
  let spaced_integer = space() * integer();
  let expression = (sym(b'(') + space().opt()) * integer_built_ins() + spaced_integer.repeat(1..)
    - space().opt()
    - sym(b')');

  expression.name("Expression").map(|(f, v)| Expression {
    function: f,
    values: v,
  })
}

fn space() -> Parser<u8, ()> {
  one_of(b" \t\r\n").repeat(1..).discard()
}

#[test]
fn parse_sum_expression() {
  let input = b"(+ 3 3 4)";
  let output = expression().parse(input);

  assert_eq!(
    output,
    Ok(Expression {
      function: BuiltIn::Add,
      values: vec![3, 3, 4]
    })
  );
}
