use std::str::from_utf8;

use pom::char_class::*;
use pom::parser::*;

use crate::parser::reserved_keywords::is_reserved_keyword;

pub fn identifier<'a>() -> Parser<'a, u8, String> {
  ((is_a(alpha) + (is_a(alphanum) | one_of(b"_-")).repeat(0..)))
    .collect()
    .convert(|id| {
        let keyword = from_utf8(id).unwrap();
        if is_reserved_keyword(keyword) {
            Err(format!("Cannot use {} as identifier because it is a reserved keyword.", keyword))
        } else {
            Ok(keyword.to_string())
        }
    })
    .name("Identifier")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid_id() {
    let input = b"memes";
    assert_eq!(Ok(String::from("memes")), identifier().parse(input));

    let input = b"im-A_1dentifier";
    assert_eq!(Ok(String::from("im-A_1dentifier")), identifier().parse(input));
  }

  #[test]
  fn fail_on_invalid_start() {
    assert!(identifier().parse(b"5v").is_err());
    assert!(identifier().parse(b"_me").is_err());
    assert!(identifier().parse(b"-x").is_err());
  }

  #[test]
  fn fail_on_keyword() {
    let input = b"if";
    assert!(identifier().parse(input).is_err())
  }
}
