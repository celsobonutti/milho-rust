extern crate pom;

use pom::parser::*;

use super::{boolean, built_in, identifier, list as list_p, number, string};
use crate::list;
use crate::types::{Atom, BuiltIn, List};

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
      | call(list_p::parser).map(Atom::List)
      | call(identifier::parser).map(Atom::Identifier)))
  .name("Atom")
  .map(|(is_quoted, atom)| {
    if is_quoted.is_some() {
      Atom::List(list![Atom::BuiltIn(BuiltIn::Quote), atom])
    } else {
      atom
    }
  })
}
