use crate::types::Atom;

pub mod atom;
pub mod boolean;
pub mod built_in;
pub mod file;
pub mod identifier;
pub mod list;
pub mod number;
pub mod space;
pub mod string;
pub mod vector;

pub fn unsafe_parse_atom(input: &str) -> Atom {
  atom::parser().parse(input.as_bytes()).unwrap()
}

pub fn unsafe_parse_file(input: &str) -> Vec<Atom> {
  file::parser().parse(input.as_bytes()).unwrap()
}
