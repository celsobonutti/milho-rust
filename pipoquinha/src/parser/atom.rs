extern crate pom;

use pom::parser::*;

use super::boolean::*;
use super::identifier::*;
use super::list::{list_parser, List};
use super::number::*;
use super::string::*;
use super::vector::*;
use crate::id;
use crate::list;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function {
  pub parameters: Vec<String>,
  pub variadic: bool,
  pub atom: Atom,
}

impl Function {
  pub fn param_len(&self) -> usize {
    self.parameters.len()
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
  List(Box<List>),
  Number(i64),
  Bool(Boolean),
  Error(String),
  Vector(Vector),
  Identifier(String),
  Function(Box<Function>),
  Str(String),
  Nil,
}

pub fn atom<'a>() -> Parser<'a, u8, Atom> {
  (sym(b'\'').opt()
    + (seq(b"Nil").map(|_| Atom::Nil)
      | number().map(Atom::Number)
      | boolean().map(Atom::Bool)
      | string().map(Atom::Str)
      | call(list_parser).map(|l| Atom::List(Box::new(l)))
      | call(vector).map(Atom::Vector)
      | call(internal_identifier).map(Atom::Identifier)))
  .name("Atom")
  .map(|(is_quoted, atom)| {
    if is_quoted.is_some() {
      Atom::List(Box::new(list![id!("quote"), atom]))
    } else {
      atom
    }
  })
}

impl Atom {
  pub fn new_function(parameters: Vec<Atom>, atom: Atom) -> Self {
    let variadic_identifier = Self::Identifier("+rest".to_string());
    let is_variadic = parameters
      .iter()
      .position(|item| item == &variadic_identifier);

    if parameters.iter().all(|item| item.is_identifier()) {
      let params = parameters
        .into_iter()
        .filter_map(|value| {
          if let Self::Identifier(id) = value {
            if id == "+rest" {
              Some("rest".to_string())
            } else {
              Some(id)
            }
          } else {
            None
          }
        })
        .collect::<Vec<String>>();

      if let Some(index) = is_variadic {
        if index == params.len() - 1 {
          return Self::Function(Box::new(Function {
            parameters: params,
            atom,
            variadic: true,
          }));
        } else {
          return Self::Error("+rest is a reserved identifier for variadics, and should only be placed at the end of your parameters".to_string());
        }
      }

      Self::Function(Box::new(Function {
        parameters: params,
        atom,
        variadic: false,
      }))
    } else {
      Self::Error("Every argument in a function must be a identifier".to_string())
    }
  }

  pub fn is_vector(&self) -> bool {
    if let Atom::Vector(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_identifier(&self) -> bool {
    if let Atom::Identifier(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_string(&self) -> bool {
    if let Atom::Str(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_error(&self) -> bool {
    if let Atom::Error(_) = self {
      true
    } else {
      false
    }
  }
}
