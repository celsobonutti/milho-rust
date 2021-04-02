use pipoquinha::arithmetic::Arithmetic::*;
use pipoquinha::atom::{
  Atom::{self, *},
  Function::*,
};
use pipoquinha::boolean::Boolean;
use pipoquinha::comparison::Comparison::*;
use pipoquinha::macros::Macro::*;

use super::VarTable;

pub fn eval(atom: Atom, variables: &VarTable) -> Atom {
  match atom {
    Expr(expr) => match expr.function {
      Ar(Add) => expr
        .values
        .into_iter()
        .fold(Number(0), |acc, val| acc.add(&eval(val, variables))),
      Ar(Mul) => expr
        .values
        .into_iter()
        .fold(Number(1), |acc, val| acc.mul(&eval(val, variables))),
      Ar(Sub) => expr
        .values
        .into_iter()
        .reduce(|acc, val| acc.add(&eval(val, variables).negate()))
        .unwrap_or(Error("Not enough arguments for subtraction".to_string())),
      Ar(Div) => expr
        .values
        .into_iter()
        .reduce(|acc, val| acc.div(&eval(val, variables)))
        .unwrap_or(Error("Not enough arguments for division".to_string())),
      Cmp(Eql) => {
        let mut arguments = expr.values.into_iter();
        if let Some(head) = arguments.next() {
          let mut res = Bool(Boolean::False);
          let base = eval(head, variables);

          while let Some(argument) = arguments.next() {
            match eval(argument, variables) {
              error @ Error(_) => return error,
              value @ _ if value != base => res = Bool(Boolean::False),
              _ => (),
            }
          }
          res
        } else {
          Error("Not enough arguments for comparison".to_string())
        }
      }
      _ => Error("Not implemented yet".to_string()),
    },
    Macro(m) => match *m {
      If {
        condition,
        if_true,
        if_false,
      } => match eval(condition, variables) {
        err @ Error(_) => err,
        Bool(Boolean::False) => eval(if_false, variables),
        _ => eval(if_true, variables),
      },
    },
    Identifier(id) => {
      if let Some(value) = variables.get(id.as_str()) {
        value.clone()
      } else {
        Error(format!("Undefined variable: {}", id))
      }
    }
    n @ Number(_) => n,
    b @ Bool(_) => b,
    e @ Error(_) => e,
    List(l) => List(l.into_iter().map(|item| eval(item, variables)).collect()),
  }
}
