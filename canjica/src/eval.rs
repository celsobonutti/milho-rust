use std::collections::HashMap;

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
      Ar(Sub) => {
        if expr.values.len() == 1 {
          expr.values.first().unwrap().negate()
        } else {
          expr
            .values
            .into_iter()
            .reduce(|acc, val| eval(acc, variables).add(&eval(val, variables).negate()))
            .unwrap_or(Error("Not enough arguments for subtraction".to_string()))
        }
      }
      Ar(Div) => expr
        .values
        .into_iter()
        .reduce(|acc, val| eval(acc, variables).div(&eval(val, variables)))
        .unwrap_or(Error("Not enough arguments for division".to_string())),
      Cmp(Eql) => {
        let mut arguments = expr.values.into_iter();
        if let Some(head) = arguments.next() {
          let mut res = Bool(Boolean::True);
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
      UserDefined(name) => {
        if let Some(atom) = variables.get(name.as_str()) {
          if let UserFunction(function) = atom {
            if function.parameters.len() == expr.values.len() {
                let mut local_table = HashMap::new();

                function.parameters.clone().into_iter().zip(expr.values.into_iter()).for_each(|(key, value)| {
                    local_table.insert(key, eval(value, variables));
                });

                local_table.extend(variables.clone()); 

                eval(function.atom.clone(), &local_table)
            } else {
              Error(format!(
                "Incorrect number of parameters for {}: expected {}, found {}.",
              function.name, function.parameters.len(), expr.values.len()))
            }
          } else {
            Error(format!("Cannot apply non-function value {}", name))
          }
        } else {
          Error(format!("Undefined function: {}", name))
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
    List(l) => List(l.into_iter().map(|item| eval(item, variables)).collect()),
    u @ UserFunction(_) => u,
    n @ Number(_) => n,
    b @ Bool(_) => b,
    e @ Error(_) => e,
  }
}
