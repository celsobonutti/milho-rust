extern crate pom;

use pom::parser::*;

use super::identifier::*;
use super::boolean::*;
use super::number::*;
use super::vector::*;
use super::list::{List, list_parser};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub expression: List
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
}


pub fn atom<'a>() -> Parser<'a, u8, Atom> {
  number().map(Atom::Number)
    | boolean().map(Atom::Bool)
    | call(list_parser).map(|l| Atom::List(Box::new(l)))
    | call(vector).map(Atom::Vector)
    | call(internal_identifier).map(Atom::Identifier)
}

