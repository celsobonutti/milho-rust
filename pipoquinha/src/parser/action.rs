use pom::parser::*;

use crate::{atom::{Atom, atom}, def::{Variable, def}, defn::{UserFunction, defn}};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
   Atom(Atom),
   FunctionDefinition(UserFunction),
   VariableDefinition(Variable)
}

pub fn action<'a>() -> Parser<'a, u8, Action> {
    use Action::*;

     atom().map(Atom)
    | defn().map(FunctionDefinition)
    | def().map(VariableDefinition)
}
