extern crate pom;

use pom::parser::*;

use super::boolean::*;
use super::identifier::*;
use super::list::{list_parser, List};
use super::number::*;
use super::vector::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function {
  pub parameters: Vec<String>,
  pub atom: Atom
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
  VariableInsertion(String, Box<Atom>),
}

pub fn atom<'a>() -> Parser<'a, u8, Atom> {
  number().map(Atom::Number)
    | boolean().map(Atom::Bool)
    | call(list_parser).map(|l| Atom::List(Box::new(l)))
    | call(vector).map(Atom::Vector)
    | call(internal_identifier).map(Atom::Identifier)
}

impl Atom {
  pub fn new_function(parameters: &Vec<Atom>, atom: &Atom) -> Self {
    if parameters.iter().all(|item| item.is_user_identifier()) {
      let params = parameters
        .into_iter()
        .filter_map(|value| {
          if let Self::Identifier(id) = value {
            Some(id.clone())
          } else {
            None
          }
        })
        .collect();

      Self::Function(Box::new(Function {
        parameters: params,
        atom: atom.clone(),
      }))
    } else {
      Self::Error("Every argument in a function must be a identifier".to_string())
    }
  }
}
