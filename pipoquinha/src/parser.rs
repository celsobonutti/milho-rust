extern crate pom;
mod arithmetic;
mod integer;

pub use arithmetic::*;
pub use integer::*;
use pom::parser::*;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
  Expr(Expression),
  Int(i64),
}

impl Add for Atom {
  type Output = Result<Self, ()>;

  fn add(self, other: Self) -> Result<Self, ()> {
    match (self, other) {
      (Atom::Int(x), Atom::Int(y)) => Ok(Atom::Int(x + y)),
      (_, _) => Err(()),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Expression {
  pub function: Arithmetic,
  pub values: Vec<Atom>,
}

pub fn atom<'a>() -> Parser<'a, u8, Atom> {
  integer().map(|i| Atom::Int(i)) | call(expression).map(|expr| Atom::Expr(expr))
}

fn space<'a>() -> Parser<'a, u8, ()> {
  one_of(b" \t\r\n").repeat(1..).discard()
}

pub fn expression<'a>() -> Parser<'a, u8, Expression> {
  let spaced_atom = space() * atom();
  let expression = (sym(b'(') + space().opt()) * arithmetic_ops() + spaced_atom.repeat(1..)
    - space().opt()
    - sym(b')');

  expression.name("Expression").map(|(f, v)| Expression {
    function: f,
    values: v,
  })
}

#[test]
fn parse_sum_expression() {
  use Atom::*;

  let input = b"(+ 3 3 4)";
  let output = expression().parse(input);

  assert_eq!(
    output,
    Ok(Expression {
      function: Arithmetic::Add,
      values: vec![Int(3), Int(3), Int(4)]
    })
  );
}

#[test]
fn parse_sum_within_sum() {
  use Atom::*;

  let input = b"(+ 3 (+ 5 3))";
  let output = expression().parse(input);
  let internal_sum = expression().parse(b"(+ 5 3)").unwrap();

  assert_eq!(
    Ok(Expression {
      function: Arithmetic::Add,
      values: vec![Int(3), Expr(internal_sum)]
    }),
    output
  )
}
