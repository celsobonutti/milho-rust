use pom::parser::*;

use crate::parser::atom;
use crate::parser::space::space;
use crate::types::Atom;

pub fn parser<'a>() -> Parser<'a, u8, Vec<Atom>> {
  list(atom::parser(), space())
}
