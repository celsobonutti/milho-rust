use pom::parser::*;

pub fn space<'a>() -> Parser<'a, u8, ()> {
  one_of(b" \t\r\n").repeat(1..).discard()
}
