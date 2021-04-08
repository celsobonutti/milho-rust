pub use crate::parser::atom::Atom;

#[macro_export]
macro_rules! id {
  ($name:literal) => {
    Atom::Identifier($name.to_string())
  }
}
