use pom::parser::*;

use super::atom;
use super::space::space;
use crate::types::List;

pub fn parser<'a>() -> Parser<'a, u8, List> {
  let parser =
    sym(b'(') * space().opt() * list(atom::parser(), space()) - space().opt() - sym(b')');

  parser.map(List::from_vec).name("List")
}

#[cfg(test)]
mod tests {
  use super::{parser, List};
  use crate::list;
  use crate::types::Atom::{self, *};
  use crate::types::Number;

  #[test]
  fn parse_sum_parser() {
    let input = b"(+ 3 3 4)";
    let output = parser().parse(input);

    assert_eq!(
      output,
      Ok(list![
        Identifier("+".to_string()),
        Atom::Number(Number::new(3, 1).unwrap()),
        Atom::Number(Number::new(3, 1).unwrap()),
        Atom::Number(Number::new(4, 1).unwrap())
      ])
    );
  }

  #[test]
  fn parse_sum_within_sum() {
    let input = b"(+ 3 (+ 5 3))";
    let output = parser().parse(input);
    let internal_sum = parser().parse(b"(+ 5 3)").unwrap();

    assert_eq!(
      Ok(list![
        Identifier("+".to_string()),
        Atom::Number(Number::new(3, 1).unwrap()),
        List(internal_sum)
      ]),
      output
    )
  }

  #[test]
  fn parse_function_definition() {
    let input = b"(defn add (x y) (+ x y))";

    let variables = list![Identifier("x".to_string()), Identifier("y".to_string())];
    let head = Identifier("defn".to_string());
    let function_name = Identifier("add".to_string());
    let parameters = List(variables.clone());
    let expression = variables.prepend(Identifier("+".to_string()));
    assert_eq!(
      Ok(list![head, function_name, parameters, List(expression)]),
      parser().parse(input)
    )
  }

  #[test]
  fn parse_variable_definition() {
    let input = b"(def my_variable 250)";

    let head = Identifier("def".to_string());
    let var_name = Identifier("my_variable".to_string());
    let value = Atom::Number(Number::new(250, 1).unwrap());

    assert_eq!(Ok(list![head, var_name, value]), parser().parse(input));
  }

  #[test]
  fn parse_anonymous_function() {
    let input = b"(fn ( var-one var-two ) (/ var-one var-two))";

    let variables = list![
      Identifier("var-one".to_string()),
      Identifier("var-two".to_string())
    ];
    let head = Identifier("fn".to_string());
    let parameters = List(variables.clone());
    let expression = List(variables.clone().prepend(Identifier("/".to_string())));

    assert_eq!(
      Ok(list![head, parameters, expression]),
      parser().parse(input)
    );
  }

  #[test]
  fn parse_local_variables() {
    let input = b"(let (x 2 y 9) (* x y 10))";

    let head = Identifier("let".to_string());

    let local_variables = List(list![
      Atom::Identifier("x".to_string()),
      Atom::unsafe_number(2, 1),
      Atom::Identifier("y".to_string()),
      Atom::unsafe_number(9, 1)
    ]);

    let expression = list![
      Identifier("*".to_string()),
      Identifier("x".to_string()),
      Identifier("y".to_string()),
      Atom::unsafe_number(10, 1)
    ];
    assert_eq!(
      Ok(list![head, local_variables, List(expression)]),
      parser().parse(input)
    );
  }

  #[test]
  fn empty_list() {
    let input = b"()";

    assert_eq!(Ok(list![]), parser().parse(input));
  }
}
