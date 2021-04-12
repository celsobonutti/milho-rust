use std::str::from_utf8;

use pom::parser::*;

pub const BUILT_INS: [&[u8]; 25] = [
  b".__add__",
  b".__mul__",
  b".__div__",
  b".__negate__",
  b".__eq__",
  b".__def__",
  b".__defn__",
  b".__defmacro__",
  b".__fn__",
  b".__let__",
  b".__if__",
  b".__read__",
  b".__eval__",
  b".__print__",
  b".__loop__",
  b".__do__",
  b".__not__",
  b".__head__",
  b".__tail__",
  b".__concat__",
  b".__cons__",
  b".__make-list__",
  b".__car__",
  b".__cdr__",
  b".__quote__",
];

pub fn parser<'a>() -> Parser<'a, u8, String> {
  BUILT_INS
    .iter()
    .map(|x| seq(*x))
    .reduce(|acc, x| acc | x)
    .unwrap()
    .collect()
    .map(|n| from_utf8(n).unwrap().to_string())
    .name("Buil-in")
}
