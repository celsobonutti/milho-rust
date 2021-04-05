use std::str::from_utf8;

use pom::char_class::*;
use pom::parser::*;

const BUILT_IN_FUNCTIONS: [&str; 15] = [
  "if", "def", "defn", "+", "-", "/", "*", ":", "=", "/=", ">", "<", ">=", "let", "fn",
];

pub fn valid_id<'a>() -> Parser<'a, u8, &'a [u8]> {
  (is_a(alpha) + (is_a(alphanum) | one_of(b"_-")).repeat(0..)).collect()
}

pub fn user_identifier<'a>() -> Parser<'a, u8, String> {
  valid_id()
    .convert(|id| {
      let keyword = from_utf8(id).unwrap();
      if BUILT_IN_FUNCTIONS.contains(&keyword) {
        Err(format!(
          "Cannot use {} as identifier because it is a reserved keyword.",
          keyword
        ))
      } else {
        Ok(keyword.to_string())
      }
    })
    .name("Identifier")
}

pub fn internal_identifier<'a>() -> Parser<'a, u8, String> {
  BUILT_IN_FUNCTIONS
    .iter()
    .fold(valid_id(), |parser, builtin_name| {
      parser | seq(builtin_name.as_bytes())
    })
    .collect()
    .map(|word| from_utf8(word).unwrap().to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid_id() {
    let input = b"memes";
    assert_eq!(Ok(String::from("memes")), user_identifier().parse(input));

    let input = b"im-A_1dentifier";
    assert_eq!(
      Ok(String::from("im-A_1dentifier")),
      user_identifier().parse(input)
    );
  }

  #[test]
  fn fail_on_invalid_start() {
    assert!(user_identifier().parse(b"5v").is_err());
    assert!(user_identifier().parse(b"_me").is_err());
    assert!(user_identifier().parse(b"-x").is_err());
  }

  #[test]
  fn user_parser_fails_on_builtin() {
    let input = b"if";
    assert!(user_identifier().parse(input).is_err())
  }

  #[test]
  fn internal_parsers_builtin() {
    let input = b"if";

    assert_eq!(Ok(String::from("if")), internal_identifier().parse(input))
  }
}
