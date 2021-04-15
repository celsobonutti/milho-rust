use crate::types::{Atom, Boolean::*, Number};

impl Atom {
  pub fn add(&self, other: &Self) -> Self {
    match (self, other) {
      (Atom::Number(x), Atom::Number(y)) => Atom::Number(*x + *y),
      (e @ Atom::Error(_), _) => e.clone(),
      (_, e @ Atom::Error(_)) => e.clone(),
      (_, _) => Atom::Error("Cannot add non-numeric values".to_string()),
    }
  }

  pub fn mul(&self, other: &Self) -> Self {
    match (self, other) {
      (Atom::Number(x), Atom::Number(y)) => Atom::Number(*x * *y),
      (_, _) => Atom::Error("Cannot multiply non-numeric values".to_string()),
    }
  }

  pub fn div(&self, other: &Self) -> Self {
    match (self, other) {
      (_, Atom::Number(x)) if x.is_zero() => Atom::Error("Cannot divide by zero".to_string()),
      (Atom::Number(x), Atom::Number(y)) => Atom::Number(*x / *y),
      (_, _) => Atom::Error("Cannot divide non-numeric values".to_string()),
    }
  }

  pub fn negate(&self) -> Self {
    if let Atom::Number(x) = self {
      Atom::Number(-*x)
    } else {
      Atom::Error("Cannot negate non-numeric value".to_string())
    }
  }

  pub fn invert(&self) -> Self {
    match self {
      Atom::Number(Number(0, _)) => Atom::Error("Tried to invert 0".to_string()),
      Atom::Number(x) => Atom::Number(x.invert()),
      _ => Atom::Error("Cannot invert non-numeric value".to_string()),
    }
  }

  pub fn eq(&self, other: &Self) -> Self {
    if self == other {
      Atom::Bool(True)
    } else {
      Atom::Bool(False)
    }
  }
}
