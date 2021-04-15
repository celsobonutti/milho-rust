use pom::parser::*;

use crate::types::BuiltIn;

pub const BUILT_INS: [&[u8]; 24] = [
  b".__add__",
  b".__mul__",
  b".__negate__",
  b".__invert__",
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
  b".__cons__",
  b".__make-list__",
  b".__car__",
  b".__cdr__",
  b".__quote__",
];

pub fn parser<'a>() -> Parser<'a, u8, BuiltIn> {
  seq(b".__add__").map(|_| BuiltIn::Add)
    | seq(b".__mul__").map(|_| BuiltIn::Mul)
    | seq(b".__negate__").map(|_| BuiltIn::Negate)
    | seq(b".__invert__").map(|_| BuiltIn::Invert)
    | seq(b".__eq__").map(|_| BuiltIn::Eql)
    | seq(b".__def__").map(|_| BuiltIn::Def)
    | seq(b".__defn__").map(|_| BuiltIn::Defn)
    | seq(b".__defmacro__").map(|_| BuiltIn::Defmacro)
    | seq(b".__fn__").map(|_| BuiltIn::Fun)
    | seq(b".__let__").map(|_| BuiltIn::Let)
    | seq(b".__if__").map(|_| BuiltIn::If)
    | seq(b".__read__").map(|_| BuiltIn::Read)
    | seq(b".__eval__").map(|_| BuiltIn::Eval)
    | seq(b".__print__").map(|_| BuiltIn::Print)
    | seq(b".__loop__").map(|_| BuiltIn::Loop)
    | seq(b".__do__").map(|_| BuiltIn::Do)
    | seq(b".__not__").map(|_| BuiltIn::Not)
    | seq(b".__cons__").map(|_| BuiltIn::Cons)
    | seq(b".__make-list__").map(|_| BuiltIn::MakeList)
    | seq(b".__car__").map(|_| BuiltIn::Car)
    | seq(b".__cdr__").map(|_| BuiltIn::Cdr)
    | seq(b".__quote__").map(|_| BuiltIn::Quote)
}
