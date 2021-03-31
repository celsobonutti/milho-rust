extern crate pom;

use pom::parser::*;

use crate::parser::atom::{atom, space, Atom};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Macro {
  If {
    condition: Atom,
    if_true: Atom,
    if_false: Atom,
  },
}

pub fn if_parser<'a>() -> Parser<'a, u8, Macro> {
  (sym(b'(') * space().opt() * seq(b"if") * space() * atom() - space() + atom() - space() + atom()
    - space().opt()
    - sym(b')'))
  .map(|((condition, if_true), if_false)| Macro::If {
    condition,
    if_true,
    if_false,
  })
}

#[cfg(test)]
mod tests {

use super::*;

use crate::boolean::Boolean;

#[test]
fn parse_if_with_static_condition() {
    let input = b"(if True 5 (+ 2 2))";
    let output = if_parser().parse(input);

    let sum_expr = atom().parse(b"(+ 2 2)").unwrap();

    let expected_result = Macro::If {
        condition: Atom::Bool(Boolean::True),
        if_true: Atom::Number(5),
        if_false: sum_expr
    };

    assert_eq!(Ok(expected_result), output);
}

#[test]
fn parse_if_with_expression_condition() {
  let input = b"(if (= 5 5) 5 6)";
  let output = if_parser().parse(input);

  let condition = atom().parse(b"(= 5 5)").unwrap();

  let expected_result = Macro::If {
    condition,
    if_true: Atom::Number(5),
    if_false: Atom::Number(6),
  };

  assert_eq!(Ok(expected_result), output)
}
}

