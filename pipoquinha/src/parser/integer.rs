extern crate pom;
use pom::parser::*;
use pom::Parser;
use std::str;

pub fn integer() -> Parser<u8, i64> {
  let sign = sym(b'-').opt();
  let number = one_of(b"1234567890").repeat(1..);

  let integer = sign + number;

  integer
    .name("Integer")
    .collect()
    .convert(str::from_utf8)
    .convert(|s| i64::from_str_radix(s, 10))
}

#[test]
fn parse_positive_number() {
  let input = b"1995";
  let output = integer().parse(input);

  assert_eq!(output, Ok(1995));
}

#[test]
fn parse_negative_number() {
  let input = b"-1995";
  let output = integer().parse(input);

  assert_eq!(output, Ok(-1995));
}

#[test]
fn parse_garbage_number() {
  let input = b"19-95";
  let output = integer().parse(input);

  assert_eq!(output, Ok(19));
}
