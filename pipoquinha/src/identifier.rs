pub use crate::types::Atom;

#[macro_export]
macro_rules! id {
  ($name:literal) => {
    Atom::Identifier($name.to_string())
  };
}
