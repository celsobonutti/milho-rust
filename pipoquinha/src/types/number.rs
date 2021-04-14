use std::mem::swap;
use std::ops::{Add, Div, Mul, Neg};
use std::{
  cmp::min,
  fmt::{Display, Formatter, Result},
};

pub fn calculate_gcd(mut u: u64, mut v: u64) -> u64 {
  if u == 0 {
    return v;
  } else if v == 0 {
    return u;
  }

  let i = u.trailing_zeros();
  u >>= i;
  let j = v.trailing_zeros();
  v >>= j;
  let k = min(i, j);

  loop {
    if u > v {
      swap(&mut u, &mut v);
    }

    v -= u;

    if v == 0 {
      return u << k;
    }

    v >>= v.trailing_zeros();
  }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Number(pub i64, pub i64);

impl Number {
  pub fn new(int: i64, frac: i64) -> Option<Self> {
    if frac != 0 {
      Some(Number(int, frac))
    } else {
      None
    }
  }

  pub fn simplify(&self) -> Self {
    let Number(integral, fractional) = self;
    let gcd = calculate_gcd((*integral).abs() as u64, (*fractional).abs() as u64);
    if (integral.is_negative() && fractional.is_negative())
      || (integral.is_positive() && fractional.is_negative())
    {
      Number(integral.neg() / gcd as i64, fractional.neg() / gcd as i64)
    } else {
      Number(integral / gcd as i64, fractional / gcd as i64)
    }
  }

  pub fn invert(&self) -> Self {
    let Number(integral, fractional) = self;
    Number(*fractional, *integral)
  }

  pub fn is_zero(&self) -> bool {
    let Number(integral, _) = self;
    *integral == 0
  }
}

impl Neg for Number {
  type Output = Self;

  fn neg(self) -> Self {
    let Number(integral, fractional) = self;

    match (integral.is_positive(), fractional.is_positive()) {
      (true, true) => Number(-integral, fractional),
      (false, false) => Number(integral, -fractional),
      (false, true) => Number(-integral, fractional),
      (true, false) => Number(-integral, -fractional),
    }
  }
}

impl Add for Number {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    let Number(first_numerator, first_denominator) = self;
    let Number(second_numerator, second_denominator) = other;
    Number(
      first_numerator * second_denominator + second_numerator * first_denominator,
      first_denominator * second_denominator,
    )
    .simplify()
  }
}

impl Mul for Number {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    let Number(first_numerator, first_denominator) = self;
    let Number(second_numerator, second_denominator) = other;

    Number(
      first_numerator * second_numerator,
      first_denominator * second_denominator,
    )
    .simplify()
  }
}

impl Div for Number {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    if let Number(0, _) = other {
      panic!("Tried to divide by zero.");
    }
    self * other.invert()
  }
}

impl Display for Number {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let Number(integral, fractional) = self;

    if *fractional == 1 {
      write!(f, "{}", integral)
    } else {
      write!(f, "{}/{}", integral, fractional)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simplification() {
    assert_eq!(Number(3, 1), Number(6, 2).simplify());

    assert_eq!(Number(7, 2), Number(7, 2).simplify());

    assert_eq!(Number(1, 3), Number(8, 24).simplify());

    assert_eq!(Number(2, 5), Number(52, 130).simplify());

    assert_eq!(Number(-2, 1), Number(-4, 2).simplify());

    assert_eq!(Number(1, 1), Number(-2, -2).simplify());

    assert_eq!(Number(-4, 3), Number(28, -21).simplify());
  }

  #[test]
  fn negate() {
    assert_eq!(Number(-1, 2), -Number(1, 2));

    assert_eq!(Number(2, 4), -Number(-2, 4));

    assert_eq!(Number(-2, 4), -Number(2, -4));

    assert_eq!(Number(-1, 2), -Number(-1, -2));
  }

  #[test]
  fn sum() {
    let sum = Number(1, 1) + Number(1, 1);

    assert_eq!(Number(2, 1), sum);

    let sum = Number(-1, 1) + Number(1, 1);

    assert_eq!(Number(0, 1), sum);

    let sum = Number(-3, 2) + Number(5, -3);

    assert_eq!(Number(-19, 6), sum);

    let sum = Number(-3, 3) + Number(10, 2);

    assert_eq!(Number(4, 1), sum);
  }

  #[test]
  fn mul() {
    let product = Number(2, 4) * Number(3, 6);

    assert_eq!(Number(1, 4), product);

    let product = Number(-1, 4) * Number(2, 3);

    assert_eq!(Number(-1, 6), product);
  }

  #[test]
  #[should_panic]
  fn div() {
    let division = Number(2, 4) / Number(3, 6);

    assert_eq!(Number(1, 1), division);

    let division = Number(3, 8) / Number(-381, 67);

    assert_eq!(Number(-67, 1016), division);

    let _ = Number(2, 2) / Number(0, 5);
  }
}
