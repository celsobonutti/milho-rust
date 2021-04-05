use crate::parser::atom::Atom::{self, *};
use crate::parser::boolean::Boolean::*;

impl Atom {
  pub fn add(&self, other: &Self) -> Self {
    match (self, other) {
      (Number(x), Number(y)) => Number(x + y),
      (e@Error(_), _) => e.clone(),
      (_, e@Error(_)) => e.clone(),
      (_, _) => Error("Cannot add non-numeric values".to_string()),
    }
  }

  pub fn mul(&self, other: &Self) -> Self {
    match (self, other) {
        (Number(x), Number(y)) => Number(x * y),
        (_, _) => Error("Cannot multiply non-numeric values".to_string()),
    }
  }

  pub fn div(&self, other: &Self) -> Self {
    match (self, other) {
        (_, Number(0)) => Error("Cannot divide by zero".to_string()),
        (Number(x), Number(y)) => Number(x / y),
        (_, _) => Error("Cannot divide non-numeric values".to_string()),
    }
  }

  pub fn negate(&self) -> Self {
    if let Number(x) = self {
      Number(-x)
    } else {
      Error("Cannot negate non-numeric value".to_string())
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
