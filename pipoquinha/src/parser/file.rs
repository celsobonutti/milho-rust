use pom::parser::*;

use crate::parser::atom::{atom, Atom};
use crate::parser::space::space;

pub fn file<'a>() -> Parser<'a, u8, Vec<Atom>> {
    list(atom(), space())
}
