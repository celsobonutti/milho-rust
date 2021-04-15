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
  use crate::types::Atom::{self, *};
  use crate::types::Number;

  #[test]
  fn parse_sum_parser() {
    let input = b"(+ 3 3 4)";
    let output = parser().parse(input);

    assert_eq!(
      output,
      Ok(List {
        head: Some(Identifier(String::from("+"))),
        tail: vec![
          Atom::Number(Number::new(3, 1).unwrap()),
          Atom::Number(Number::new(3, 1).unwrap()),
          Atom::Number(Number::new(4, 1).unwrap())
        ]
      })
    );
  }

  #[test]
  fn parse_sum_within_sum() {
    let input = b"(+ 3 (+ 5 3))";
    let output = parser().parse(input);
    let internal_sum = parser().parse(b"(+ 5 3)").unwrap();

    assert_eq!(
      Ok(List {
        head: Some(Identifier(String::from("+"))),
        tail: vec![
          Atom::Number(Number::new(3, 1).unwrap()),
          List(Box::new(internal_sum))
        ]
      }),
      output
    )
  }

  #[test]
  fn parse_function_definition() {
    let input = b"(defn add [ x y ] (+ x y))";

    let variables = vec![Identifier(String::from("x")), Identifier(String::from("y"))];
    let head = Some(Identifier(String::from("defn")));
    let function_name = Identifier(String::from("add"));
    let parameters = Vector(variables.clone());
    let expression = Box::new(List {
      head: Some(Identifier(String::from("+"))),
      tail: variables,
    });

    assert_eq!(
      Ok(List {
        head,
        tail: vec![function_name, parameters, List(expression)]
      }),
      parser().parse(input)
    )
  }

  #[test]
  fn parse_variable_definition() {
    let input = b"(def my_variable 250)";

    let head = Some(Identifier(String::from("def")));
    let var_name = Identifier(String::from("my_variable"));
    let value = Atom::Number(Number::new(250, 1).unwrap());

    assert_eq!(
      Ok(List {
        head,
        tail: vec![var_name, value]
      }),
      parser().parse(input)
    );
  }

  #[test]
  fn parse_anonymous_function() {
    let input = b"(fn [ var-one var-two ] (/ var-one var-two))";

    let variables = vec![
      Identifier(String::from("var-one")),
      Identifier(String::from("var-two")),
    ];
    let head = Some(Identifier(String::from("fn")));
    let parameters = Vector(variables.clone());
    let expression = List(Box::new(List {
      head: Some(Identifier(String::from("/"))),
      tail: variables,
    }));

    assert_eq!(
      Ok(List {
        head,
        tail: vec![parameters, expression]
      }),
      parser().parse(input)
    );
  }

  #[test]
  fn parse_local_variables() {
    let input = b"(let [ x 2 y 9 ] (* x y 10))";

    let head = Some(Identifier(String::from("let")));
    let local_variables = Vector(vec![
      Identifier(String::from("x")),
      Atom::Number(Number::new(2, 1).unwrap()),
      Identifier(String::from("y")),
      Atom::Number(Number::new(9, 1).unwrap()),
    ]);
    let expression = Box::new(List {
      head: Some(Identifier(String::from("*"))),
      tail: vec![
        Identifier(String::from("x")),
        Identifier(String::from("y")),
        Atom::Number(Number::new(10, 1).unwrap()),
      ],
    });

    assert_eq!(
      Ok(List {
        head,
        tail: vec![local_variables, List(expression)]
      }),
      parser().parse(input)
    );
  }

  #[test]
  fn empty_list() {
    let input = b"()";

    assert_eq!(
      Ok(List {
        head: None,
        tail: vec![]
      }),
      parser().parse(input)
    );
  }
}
