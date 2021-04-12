use std::str::from_utf8;

use pom::parser::*;

const INVALID_STARTS: [char; 1] = ['.'];

const INVALID_CHARACTERS: [char; 10] = ['\"', '\'', '[', ']', '(', ')', ' ', '\t', '\r', '\n'];

pub fn invalid_start(character: u8) -> bool {
  let character = character as char;
  character.is_numeric()
    || INVALID_STARTS.contains(&character)
    || INVALID_CHARACTERS.contains(&character)
}

pub fn invalid_character(character: u8) -> bool {
  INVALID_CHARACTERS.contains(&(character as char))
}

pub fn parser<'a>() -> Parser<'a, u8, String> {
  (not_a(invalid_start) + not_a(invalid_character).repeat(0..))
    .collect()
    .map(|chars| from_utf8(chars).unwrap().to_string())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::built_in::BUILT_INS;

  #[test]
  fn valid_id() {
    let input = b"memes";
    assert_eq!(Ok(String::from("memes")), parser().parse(input));

    let input = b"im-A_1dentifier";
    assert_eq!(Ok(String::from("im-A_1dentifier")), parser().parse(input));
  }

  #[test]
  fn fail_on_invalid_character() {
    assert!(parser().parse(b"5v").is_err());
    assert!(parser().parse(b"\"me").is_err());
    assert!(parser().parse(b"\'[](").is_err());
  }

  #[test]
  fn user_parser_fails_on_builtin() {
    for built_in in BUILT_INS.iter() {
      assert!(parser().parse(built_in).is_err())
    }
  }
}
