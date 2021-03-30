use crate::parser::atom::Atom::{self, *};
use crate::parser::boolean::Boolean::*;

impl Atom {
  pub fn add(&self, other: &Self) -> Self {
    match (self, other) {
      (Number(x), Number(y)) => Number(x + y),
      (_, _) => Error("Cannot add non-numeric values"),
    }
  }

  pub fn negate(&self) -> Self {
    if let Number(x) = self {
      Number(-x)
    } else {
      Error("Cannot negate non-numeric value")
    }
  }

  pub fn eq(&self, other: &Self) -> Self {
    if self == other {
      Bool(True)
    } else {
      Bool(False)
    }
  }
}
