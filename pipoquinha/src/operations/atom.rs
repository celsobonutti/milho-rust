use crate::parser::atom::Atom;
use crate::parser::identifier::is_builtin;

impl Atom {
  pub fn is_user_identifier(&self) -> bool {
    if let Atom::Identifier(id) = self {
      !is_builtin(id) 
    } else {
      false
    }
  }
}
