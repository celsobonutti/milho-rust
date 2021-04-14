use pom::parser::*;

use crate::parser::atom::{self, Atom};
use crate::parser::space::space;

pub fn parser<'a>() -> Parser<'a, u8, Vec<Atom>> {
    list(atom::parser(), space())
}
