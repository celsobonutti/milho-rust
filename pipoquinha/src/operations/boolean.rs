use crate::types::{
  Atom::{self, *},
  Boolean::*,
};

impl Atom {
  pub fn not(&self) -> Self {
    match self {
      Bool(False) => Bool(True),
      Bool(True) => Bool(False),
      _ => Error("Cannot use not on non-boolean values".to_string()),
    }
  }

  pub fn and(&self, other: &Self) -> Self {
    match (self, other) {
      (Bool(False), _) => Bool(False),
      (_, Bool(False)) => Bool(False),
      (Bool(True), Bool(True)) => Bool(True),
      _ => Error("Cannot AND non boolean values".to_string()),
    }
  }
}
