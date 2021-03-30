use pipoquinha::arithmetic::Arithmetic;
use pipoquinha::atom::{
  Atom::{self, *},
  Function::*,
};
use pipoquinha::boolean::Boolean;
use pipoquinha::macros::Macro::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum EvalError {
  NotImplemented,
  NotEnoughArguments,
  TypeError(&'static str),
  DividedByZero,
}

pub fn eval(atom: Atom) -> Atom {
  match atom {
    Expr(expr) => match expr.function {
      Ar(Arithmetic::Add) => expr.values.iter().fold(Number(0), |acc, val| acc.add(val)),
      _ => Error("Not implemented yet"),
    },
    Macro(m) => match *m {
      If {
        condition,
        if_true,
        if_false,
      } => match eval(condition) {
        err @ Error(_) => err,
        Bool(Boolean::False) => eval(if_false),
        _ => eval(if_true),
      },
    },
    Number(x) => Number(x),
    Bool(x) => Bool(x),
    Error(x) => Error(x),
    List(x) => List(x.clone()),
  }
}
