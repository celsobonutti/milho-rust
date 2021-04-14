extern crate pom;

use pom::parser::*;

use super::boolean::Boolean;
use super::list::List;
use super::vector::Vector;
use super::{boolean, built_in, identifier, list as list_p, number, string, vector};
use crate::{list, types::number::Number};

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
  Number(Number),
  Bool(Boolean),
  Error(String),
  Vector(Vector),
  Identifier(String),
  Function(Box<Function>),
  Macro(Box<Function>),
  MultiArityFn(Box<Vec<Function>>),
  Str(String),
  BuiltIn(String),
  Nil,
}

pub fn parser<'a>() -> Parser<'a, u8, Atom> {
  (sym(b'\'').opt()
    + (seq(b"Nil").map(|_| Atom::Nil)
      | number::parser().map(|v| {
        v.map_or(
          Atom::Error("Cannot create numbers with 0 as denominator".to_string()),
          Atom::Number,
        )
      })
      | boolean::parser().map(Atom::Bool)
      | string::parser().map(Atom::Str)
      | built_in::parser().map(Atom::BuiltIn)
      | call(list_p::parser).map(|l| Atom::List(Box::new(l)))
      | call(vector::parser).map(Atom::Vector)
      | call(identifier::parser).map(Atom::Identifier)))
  .name("Atom")
  .map(|(is_quoted, atom)| {
    if is_quoted.is_some() {
      Atom::List(Box::new(list![
        Atom::BuiltIn(".__quote__".to_string()),
        atom
      ]))
    } else {
      atom
    }
  })
}

impl Atom {
  pub fn new_function(parameters: Vec<Atom>, atom: Atom, is_macro: bool) -> Self {
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
          if is_macro {
            return Self::Macro(Box::new(Function {
              parameters: params,
              atom,
              variadic: true,
            }));
          } else {
            return Self::Function(Box::new(Function {
              parameters: params,
              atom,
              variadic: true,
            }));
          }
        } else {
          return Self::Error("+rest is a reserved identifier for variadics, and should only be placed at the end of your parameters".to_string());
        }
      }

      if is_macro {
        Self::Macro(Box::new(Function {
          parameters: params,
          atom,
          variadic: false,
        }))
      } else {
        Self::Function(Box::new(Function {
          parameters: params,
          atom,
          variadic: false,
        }))
      }
    } else {
      Self::Error("Every argument in a function must be a identifier".to_string())
    }
  }

  pub fn is_list(&self) -> bool {
    if let Atom::List(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_vector(&self) -> bool {
    if let Atom::Vector(_) = self {
      true
    } else {
      false
    }
  }

  pub fn unwrap_vector(self) -> Vec<Atom> {
    if let Atom::Vector(v) = self {
      v
    } else {
      panic!("Trying to unwrap a vector from a non-vector atom");
    }
  }

  pub fn is_identifier(&self) -> bool {
    if let Atom::Identifier(_) = self {
      true
    } else {
      false
    }
  }

  pub fn unwrap_id(self) -> String {
    if let Atom::Identifier(id) = self {
      id
    } else {
      panic!("Trying to unwrap an id from a non-identifier atom");
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

  pub fn unsafe_number(x: i64, y: i64) -> Self {
    Self::Number(Number(x, y))
  }

  pub fn make_boolean(x: bool) -> Self {
    if x {
      Self::Bool(Boolean::True)
    } else {
      Self::Bool(Boolean::False)
    }
  }

  pub fn make_string(x: &str) -> Self {
    Self::Str(x.to_string())
  }
}
