extern crate pom;
use pom::parser::*;
use std::str;

use crate::types::number::Number;

pub fn parser<'a>() -> Parser<'a, u8, Option<Number>> {
  let sign = sym(b'-').opt();
  let number = one_of(b"1234567890").repeat(1..);
  let number2 = one_of(b"1234567890").repeat(1..);

  let number = (sign + number).collect() + (sym(b'/') * number2.collect()).opt();

  number.name("Number").map(|(integer, fractional)| {
    let int = i64::from_str_radix(str::from_utf8(integer).unwrap(), 10).unwrap();
    let frac = fractional
      .map(|v| i64::from_str_radix(str::from_utf8(v).unwrap(), 10).unwrap())
      .unwrap_or(1);
    Number::new(int, frac)
  })
}

#[test]
fn parse_positive_number() {
  let input = b"1995";
  let output = parser().parse(input);

  assert_eq!(output, Ok(Number::new(1995, 1)));
}

#[test]
fn parse_negative_number() {
  let input = b"-1995";
  let output = parser().parse(input);

  assert_eq!(output, Ok(Number::new(-1995, 1)));
}

#[test]
fn parse_fractional_number() {
  let input = b"19/95";
  let output = parser().parse(input);

  assert_eq!(output, Ok(Number::new(19, 95)));
}
