use self::atom::{Atom};

pub mod boolean;
pub mod number;
pub mod atom;
pub mod vector;
pub mod identifier;
pub mod list;
pub mod space;
pub mod file;
pub mod string;
pub mod built_in;

pub fn unsafe_parse_atom(input: &str) -> Atom {
    atom::parser().parse(input.as_bytes()).unwrap()
}

pub fn unsafe_parse_file(input: &str) -> Vec<Atom> {
    file::parser().parse(input.as_bytes()).unwrap()
}
