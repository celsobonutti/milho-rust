use pom::parser::*;

pub fn parser<'a>() -> Parser<'a, u8, String> {
  let special_char = sym(b'\\')
    | sym(b'/')
    | sym(b'"')
    | sym(b'b').map(|_| b'\x08')
    | sym(b'f').map(|_| b'\x0C')
    | sym(b'n').map(|_| b'\n')
    | sym(b'r').map(|_| b'\r')
    | sym(b't').map(|_| b'\t');

  let escape_sequence = sym(b'\\') * special_char;

  let string = sym(b'"') * (none_of(b"\\\"") | escape_sequence).repeat(0..) - sym(b'"');

  string.convert(String::from_utf8).name("String")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_string() {
    let input = b"\"memes\"";

    assert_eq!(Ok("memes".to_string()), parser().parse(input))
  }

  #[test]
  fn empty_string() {
    let input = b"\"\"";

    assert_eq!(Ok("".to_string()), parser().parse(input))
  }
}
