extern crate pom;

use pom::parser::*;

use crate::macros::{Macro, if_parser};

use super::boolean::*;
use super::arithmetic::*;
use super::comparison::*;
use super::number::*;
use super::list::{List, list as list_parser};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
  Expr(Expression),
  Number(i64),
  Bool(Boolean),
  Error(&'static str),
  List(List),
  Macro(Box<Macro>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Function {
  Ar(Arithmetic),
  Cmp(Comparison),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Expression {
  pub function: Function,
  pub values: Vec<Atom>,
}

pub fn function<'a>() -> Parser<'a, u8, Function> {
  comparison_functions().map(|c| Function::Cmp(c)) | arithmetic_ops().map(|a| Function::Ar(a))
}

pub fn atom<'a>() -> Parser<'a, u8, Atom> {
  number().map(|i| Atom::Number(i))
    | call(expression).map(|expr| Atom::Expr(expr))
    | boolean().map(|b| Atom::Bool(b))
    | call(list_parser).map(|l| Atom::List(l))
    | call(if_parser).map(|m| Atom::Macro(Box::new(m)))
}

pub fn space<'a>() -> Parser<'a, u8, ()> {
  one_of(b" \t\r\n").repeat(1..).discard()
}

pub fn expression<'a>() -> Parser<'a, u8, Expression> {
  let spaced_atom = space() * atom();
  let expression =
    (sym(b'(') + space().opt()) * function() + spaced_atom.repeat(1..) - space().opt() - sym(b')');

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
      function: Function::Ar(Arithmetic::Add),
      values: vec![Number(3), Number(3), Number(4)]
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
      function: Function::Ar(Arithmetic::Add),
      values: vec![Number(3), Expr(internal_sum)]
    }),
    output
  )
}