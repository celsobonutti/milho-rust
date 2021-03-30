use pipoquinha::arithmetic::Arithmetic;
use pipoquinha::atom::{
  Atom::{self, *},
  Function::*,
};
use pipoquinha::boolean::Boolean;
use pipoquinha::macros::Macro::*;

pub fn eval(atom: Atom) -> Atom {
  match atom {
    Expr(expr) => match expr.function {
      Ar(Arithmetic::Add) => expr.values.into_iter().fold(Number(0), |acc, val| acc.add(&eval(val))),
      Ar(Arithmetic::Mul) => expr.values.into_iter().fold(Number(1), |acc, val| acc.mul(&eval(val))),
      Ar(Arithmetic::Sub) => expr.values.into_iter().reduce(|acc, val| acc.add(&eval(val).negate())).unwrap_or(Error("Not enough arguments for subtraction")),
      Ar(Arithmetic::Div) => expr.values.into_iter().reduce(|acc, val| acc.div(&eval(val))).unwrap_or(Error("Not enough arguments for division")),
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
    n@Number(_) => n,
    b@Bool(_) => b,
    e@Error(_) => e,
    l@List(_) => l,
  }
}
